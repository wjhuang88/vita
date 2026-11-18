//! Deliberately narrow VM config projection; never load skills, hooks or host tools.
use std::{fs::File, io::Read, sync::Arc};
use serde::Deserialize;
use talos_config::{ProviderTimeoutConfig, ReasoningOptions};
use talos_provider::{openai::OpenAIProvider, OpenAIResponsesProvider};
use talos_runtime::LanguageModel;

const CONFIG: &str = "/run/vita-model/config.json";
const LIMIT: u64 = 32768;
const INVALID: &str = "model config unavailable/invalid; recreate the private readonly share";

// No Debug: this object contains a credential.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelConfig {
    protocol: String,
    model: String,
    base_url: String,
    api_key: String,
    #[serde(default)]
    reasoning: Option<ReasoningOptions>,
    #[serde(default)]
    timeout: ProviderTimeoutConfig,
}

fn parse(bytes: &[u8]) -> Result<ModelConfig, &'static str> {
    if bytes.len() > LIMIT as usize { return Err(INVALID); }
    let config: ModelConfig = serde_json::from_slice(bytes).map_err(|_| INVALID)?;
    let url = url::Url::parse(&config.base_url).map_err(|_| INVALID)?;
    if url.scheme() != "https" || url.host_str().is_none() || !url.username().is_empty()
        || url.password().is_some() || url.query().is_some() || url.fragment().is_some()
        || !matches!(config.protocol.as_str(), "openai-chat" | "openai-responses")
        || config.model.is_empty() || config.model.len() > 200
        || config.model.chars().any(char::is_control)
        || config.api_key.trim().is_empty() || config.api_key.len() > 8192
        || config.api_key.chars().any(char::is_control)
    { return Err(INVALID); }
    Ok(config)
}

pub fn load_provider() -> Result<Arc<dyn LanguageModel>, &'static str> {
    let mut bytes = Vec::new();
    File::open(CONFIG).map_err(|_| INVALID)?.take(LIMIT + 1)
        .read_to_end(&mut bytes).map_err(|_| INVALID)?;
    let config = parse(&bytes)?;
    // Operator timeouts are honored within this bounded test session.
    let mut timeout = config.timeout;
    timeout.dispatch_timeout_secs = timeout.dispatch_timeout_secs.clamp(1, 120);
    timeout.first_packet_timeout_secs = timeout.first_packet_timeout_secs.clamp(1, 120);
    timeout.stream_idle_timeout_secs = timeout.stream_idle_timeout_secs.clamp(1, 120);
    timeout.max_attempts = timeout.max_attempts.min(2);
    timeout.backoff_base_ms = timeout.backoff_base_ms.min(2000);
    timeout.backoff_max_ms = timeout.backoff_max_ms.min(5000);
    let provider: Arc<dyn LanguageModel> = match config.protocol.as_str() {
        "openai-chat" => Arc::new(OpenAIProvider::new(config.api_key, config.model)
            .with_base_url(config.base_url).with_reasoning(config.reasoning, Some(4096))
            .with_timeout_config(timeout)),
        "openai-responses" => Arc::new(OpenAIResponsesProvider::new(config.api_key, config.model)
            .with_base_url(config.base_url).with_reasoning(config.reasoning, Some(4096))
            .with_timeout_config(timeout)),
        _ => return Err(INVALID),
    };
    Ok(provider)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn config() -> serde_json::Value {
        serde_json::json!({"protocol":"openai-chat","model":"test-model",
            "base_url":"https://example.invalid/v1","api_key":"CANARY_SECRET"})
    }
    #[test]
    fn rejects_invalid_and_never_echoes_secrets() {
        for endpoint in ["http://example.invalid", "https://user:CANARY_SECRET@example.invalid", "https://example.invalid?key=CANARY_SECRET"] {
            let mut c = config(); c["base_url"] = endpoint.into();
            assert_eq!(parse(&serde_json::to_vec(&c).unwrap()).err(), Some(INVALID));
        }
        for bytes in [b"{CANARY_SECRET".to_vec(), vec![b'x'; 32769]] {
            let err = parse(&bytes).err().unwrap();
            assert!(!err.contains("CANARY_SECRET"));
        }
        let mut c = config(); c["hooks"] = serde_json::json!(["arbitrary-host-command"]);
        assert!(parse(&serde_json::to_vec(&c).unwrap()).is_err());
        let mut c = config(); c["api_key"] = "secret\r\nheader".into();
        assert!(parse(&serde_json::to_vec(&c).unwrap()).is_err());
    }
    #[test]
    fn accepts_supported_minimal_projection() {
        assert!(parse(&serde_json::to_vec(&config()).unwrap()).is_ok());
    }
}
