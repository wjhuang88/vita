use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::{Value, json};
use talos_runtime::{
    AgentEvent, AgentTool, CapabilityProbe, LanguageModel, Message, PermissionDecision,
    PermissionRule, ProtocolCapabilities, ProviderError, ProviderResult, Receiver, RuntimeBuilder,
    RuntimeTurnCompletionStatus, StopReason, ToolCall, ToolNature, ToolProvenance, ToolResult, Usage,
    collect_until_turn_completed,
};

#[derive(Default)]
struct DeterministicProvider {
    responses: Mutex<VecDeque<(AgentEvent, StopReason)>>,
}

impl DeterministicProvider {
    fn new() -> Self {
        Self::default()
    }

    fn with_tool_call(mut self, name: &str, input: Value) -> Self {
        self.responses
            .get_mut()
            .expect("prototype provider mutex")
            .push_back((
                AgentEvent::ToolCall {
                    call: ToolCall {
                        id: "vita-i001-system-summary".into(),
                        name: name.into(),
                        input,
                    },
                    provenance: ToolProvenance::Native,
                    summary_fields: vec![],
                },
                StopReason::ToolUse,
            ));
        self
    }

    fn with_response(mut self, text: &str) -> Self {
        self.responses
            .get_mut()
            .expect("prototype provider mutex")
            .push_back((
                AgentEvent::TextDelta { delta: text.into() },
                StopReason::EndTurn,
            ));
        self
    }
}

#[async_trait]
impl LanguageModel for DeterministicProvider {
    fn protocol_capabilities(&self) -> CapabilityProbe {
        CapabilityProbe::Known(ProtocolCapabilities {
            native_tools: true,
            compatibility: false,
        })
    }

    async fn stream(&self, _messages: &[Message]) -> ProviderResult<Receiver<AgentEvent>> {
        let (event, stop_reason) = self
            .responses
            .lock()
            .map_err(|_| ProviderError::InvalidResponse("prototype provider mutex poisoned".into()))?
            .pop_front()
            .ok_or_else(|| ProviderError::InvalidResponse("prototype response exhausted".into()))?;

        let (tx, rx) = tokio::sync::mpsc::channel(3);
        for event in [
            AgentEvent::TurnStart,
            event,
            AgentEvent::TurnEnd {
                stop_reason,
                usage: Usage::default(),
            },
        ] {
            tx.try_send(event).map_err(|_| {
                ProviderError::InvalidResponse("prototype event channel unavailable".into())
            })?;
        }
        Ok(rx)
    }
}

struct SystemSummary {
    executed: Arc<AtomicBool>,
}

#[async_trait]
impl AgentTool for SystemSummary {
    fn name(&self) -> &str {
        "system_summary"
    }

    fn description(&self) -> &str {
        "Returns bounded read-only facts about the current Linux environment without invoking a shell"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        })
    }

    async fn execute(&self, _input: Value) -> ToolResult {
        self.executed.store(true, Ordering::SeqCst);

        let facts = vita_system_facts::inspect_linux();
        let summary = json!({
            "os_release": facts.os_release,
            "hostname": facts.hostname,
            "kernel": facts.kernel_release,
            "architecture": facts.architecture,
            "family": facts.family,
            "uptime_seconds": facts.uptime_seconds,
            "memory_total_kib": facts.memory_total_kib,
        });

        ToolResult::success(summary.to_string())
    }

    fn is_read_only(&self) -> bool {
        true
    }

    fn nature(&self) -> ToolNature {
        ToolNature::Read
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executed = Arc::new(AtomicBool::new(false));
    let provider: Arc<dyn LanguageModel> = Arc::new(
        DeterministicProvider::new()
            .with_tool_call("system_summary", json!({}))
            .with_response("Vita structured system inspection completed."),
    );

    let mut runtime = RuntimeBuilder::new()
        .provider(provider)
        .workspace_root(".")
        .tool(Arc::new(SystemSummary {
            executed: executed.clone(),
        }))
        .permission_rule(PermissionRule::new_nature(
            ToolNature::Read,
            None,
            None,
            PermissionDecision::Allow,
        ))
        .build()?;

    runtime.submit("Inspect this system using the structured system capability.").await?;
    let status = collect_until_turn_completed(&mut runtime)
        .await
        .ok_or_else(|| std::io::Error::other("runtime ended before turn completion"))?;

    if !matches!(status, RuntimeTurnCompletionStatus::Success { .. }) {
        return Err(std::io::Error::other(format!(
            "unexpected runtime completion: {status:?}"
        ))
        .into());
    }
    if !executed.load(Ordering::SeqCst) {
        return Err(std::io::Error::other("structured system capability was not executed").into());
    }

    runtime.shutdown().await?;
    println!("VITA_I001_TALOS_RUNTIME_OK");
    Ok(())
}
