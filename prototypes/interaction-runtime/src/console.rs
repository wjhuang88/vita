//! Native host console proof over supported Talos Runtime SDK.
//! Offline model fixture only: NOT a complete Vita Live OS session.
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, BufRead, Write};
use std::sync::{Arc, atomic::AtomicBool};
use std::time::Duration;

use async_trait::async_trait;
use serde_json::json;
use talos_runtime::{
    AgentEvent, CapabilityProbe, LanguageModel, Message, PermissionDecision, PermissionRule,
    ProtocolCapabilities, ProviderError, ProviderResult, Receiver, RuntimeBuilder, RuntimeHandle,
    SessionEvent, StopReason, ToolCall, ToolNature, ToolProvenance, TurnCompletionStatus,
    TurnEventPayload, Usage,
};
use tokio::sync::mpsc;

use crate::SystemSummary;

const INPUT_LIMIT: usize = 16384;
const TURN_LIMIT: Duration = Duration::from_secs(30);

#[derive(Debug, Eq, PartialEq)]
enum Command { Empty, Quit, Cancel, Help, Prompt(String), TooLong }

fn parse(line: &str) -> Command {
    let text = line.trim();
    if text.len() > INPUT_LIMIT { return Command::TooLong; }
    match text {
        "" => Command::Empty,
        ":quit" | ":exit" => Command::Quit,
        ":cancel" => Command::Cancel,
        ":help" => Command::Help,
        _ => Command::Prompt(text.to_string()),
    }
}

struct FixtureProvider;

#[async_trait]
impl LanguageModel for FixtureProvider {
    fn protocol_capabilities(&self) -> CapabilityProbe {
        CapabilityProbe::Known(ProtocolCapabilities { native_tools: true, compatibility: false })
    }
    async fn stream(&self, messages: &[Message]) -> ProviderResult<Receiver<AgentEvent>> {
        let followup = matches!(messages.last(), Some(Message::Tool { .. }));
        let user = messages.iter().rev().find_map(|m| match m {
            Message::User { content } => Some(content.as_str()),
            _ => None,
        });
        if !followup && user == Some("demo:error") {
            return Err(ProviderError::InvalidResponse("intentional offline fixture error".into()));
        }
        if !followup && user == Some("demo:slow") {
            tokio::time::sleep(Duration::from_secs(15)).await;
        }
        let (tx, rx) = tokio::sync::mpsc::channel(4);
        let events = if followup {
            vec![
                AgentEvent::TurnStart,
                AgentEvent::TextDelta { delta: "Vita fixture: system inspection completed.".into() },
                AgentEvent::TurnEnd { stop_reason: StopReason::EndTurn, usage: Usage::default() },
            ]
        } else {
            vec![
                AgentEvent::TurnStart,
                AgentEvent::ToolCall {
                    call: ToolCall {
                        id: "vita-console-tool".into(),
                        name: "system_summary".into(),
                        input: json!({}),
                    },
                    provenance: ToolProvenance::Native,
                    summary_fields: vec![],
                },
                AgentEvent::TurnEnd { stop_reason: StopReason::ToolUse, usage: Usage::default() },
            ]
        };
        for event in events {
            tx.try_send(event).map_err(|_| {
                ProviderError::InvalidResponse("offline receiver closed".into())
            })?;
        }
        Ok(rx)
    }
}

#[derive(Default)]
struct Renderer { streamed_text: bool }
impl Renderer {
    /// Consume canonical TurnEvent only; the legacy AgentEvent may duplicate it.
    /// Never expose thinking content, raw tool arguments or tool outputs.
    fn event(&mut self, input: SessionEvent, out: &mut impl Write) -> io::Result<bool> {
        match input {
            SessionEvent::TurnEvent { payload: TurnEventPayload::Progress { event }, .. } => {
                match event {
                    AgentEvent::TextDelta { delta } => {
                        write!(out, "{delta}")?;
                        self.streamed_text = true;
                        out.flush()?;
                    }
                    AgentEvent::ToolCall { call, .. } => writeln!(out, "\n[tool] {}", call.name)?,
                    _ => {}
                }
            }
            SessionEvent::TurnEvent { payload: TurnEventPayload::Completed { status }, .. } => {
                match status {
                    TurnCompletionStatus::Success { final_text, .. } => {
                        if !self.streamed_text { write!(out, "{final_text}")?; }
                        writeln!(out, "\n[turn:ok]")?;
                    }
                    TurnCompletionStatus::Cancelled => writeln!(out, "\n[turn:cancelled]")?,
                    TurnCompletionStatus::Error { .. } => writeln!(out, "\n[turn:error]")?,
                }
                out.flush()?;
                return Ok(true);
            }
            SessionEvent::Error { .. } => writeln!(out, "[session:error]")?,
            _ => {}
        }
        Ok(false)
    }
}

fn help(out: &mut impl Write) -> io::Result<()> {
    writeln!(out, "Commands: :help, :cancel (active turn), :quit / :exit")?;
    writeln!(out, "Offline test prompts: inspect, demo:error, demo:slow")
}

struct ConsoleInput {
    rx: mpsc::UnboundedReceiver<io::Result<String>>,
    pending: VecDeque<String>,
    eof: bool,
    exit_after_turn: bool,
}

impl ConsoleInput {
    fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        // Blocking stdin is isolated to an OS thread and cannot hold a Tokio worker.
        std::thread::spawn(move || {
            for line in io::stdin().lock().lines() {
                if tx.send(line).is_err() { break; }
            }
        });
        Self { rx, pending: VecDeque::new(), eof: false, exit_after_turn: false }
    }

    async fn prompt(&mut self, out: &mut impl Write) -> io::Result<Option<String>> {
        if let Some(line) = self.pending.pop_front() { return Ok(Some(line)); }
        loop {
            if self.eof { return Ok(None); }
            write!(out, "vita> ")?;
            out.flush()?;
            tokio::select! {
                line = self.rx.recv() => match line {
                    Some(Ok(line)) => match parse(&line) {
                        Command::Prompt(text) => return Ok(Some(text)),
                        Command::Quit => return Ok(None),
                        Command::Cancel => writeln!(out, "[no active turn]")?,
                        Command::Help => help(out)?,
                        Command::TooLong => writeln!(out, "[input rejected]")?,
                        Command::Empty => {}
                    },
                    Some(Err(err)) => return Err(err),
                    None => self.eof = true,
                },
                signal = tokio::signal::ctrl_c() => {
                    signal?;
                    writeln!(out, "\n[exit requested]")?;
                    return Ok(None);
                }
            }
        }
    }

    async fn turn(&mut self, runtime: &mut RuntimeHandle, out: &mut impl Write)
        -> Result<(), Box<dyn Error>>
    {
        let mut renderer = Renderer::default();
        let deadline = tokio::time::sleep(TURN_LIMIT);
        tokio::pin!(deadline);
        let mut interrupted = false;
        loop {
            tokio::select! {
                item = runtime.next_event() => {
                    let event = item.ok_or("Talos event stream ended before turn completion")?;
                    if renderer.event(event, out)? { return Ok(()); }
                }
                item = self.rx.recv(), if !self.eof => match item {
                    Some(Ok(line)) => match parse(&line) {
                        Command::Prompt(text) => {
                            if !self.exit_after_turn {
                                self.pending.push_back(text);
                                writeln!(out, "\n[queued: next turn]")?;
                            }
                        }
                        Command::Cancel if !interrupted => {
                            runtime.interrupt().await?;
                            interrupted = true;
                            writeln!(out, "\n[interrupt requested]")?;
                        }
                        Command::Quit => {
                            self.exit_after_turn = true;
                            writeln!(out, "\n[exit after queued turns]")?;
                        }
                        Command::Help => help(out)?,
                        Command::TooLong => writeln!(out, "\n[input rejected]")?,
                        _ => {}
                    },
                    Some(Err(err)) => return Err(err.into()),
                    None => {
                        self.eof = true;
                        self.exit_after_turn = true;
                    }
                },
                signal = tokio::signal::ctrl_c(), if !interrupted => {
                    signal?;
                    runtime.interrupt().await?;
                    interrupted = true;
                    writeln!(out, "\n[interrupt requested]")?;
                }
                _ = &mut deadline => {
                    if !interrupted { runtime.interrupt().await?; }
                    return Err("offline fixture turn timeout (30 s)".into());
                }
            }
        }
    }
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let mut out = io::stdout().lock();
    writeln!(out, "Vita offline console fixture — not a production model or Live OS")?;
    help(&mut out)?;

    let provider: Arc<dyn LanguageModel> = Arc::new(FixtureProvider);
    let mut runtime = RuntimeBuilder::new()
        .provider(provider)
        .workspace_root(".")
        .tool(Arc::new(SystemSummary { executed: Arc::new(AtomicBool::new(false)) }))
        .permission_rule(PermissionRule::new_nature(
            ToolNature::Read, None, None, PermissionDecision::Allow
        ))
        .build()?;
    let mut input = ConsoleInput::new();
    let mut result: Result<(), Box<dyn Error>> = Ok(());
    loop {
        if input.exit_after_turn && input.pending.is_empty() { break; }
        let prompt = match input.prompt(&mut out).await? {
            Some(value) => value,
            None => break,
        };
        if let Err(err) = runtime.submit(prompt).await {
            result = Err(err.into());
            break;
        }
        if let Err(err) = input.turn(&mut runtime, &mut out).await {
            result = Err(err);
            break;
        }
    }
    if let Err(err) = runtime.shutdown().await {
        writeln!(out, "[shutdown:incomplete]")?;
        if result.is_ok() { result = Err(err.into()); }
    }
    if result.is_ok() { writeln!(out, "VITA_CONSOLE_EXIT_OK")?; }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_parser() {
        assert_eq!(parse(""), Command::Empty);
        assert_eq!(parse(" :cancel "), Command::Cancel);
        assert_eq!(parse(":quit"), Command::Quit);
        assert_eq!(parse(":help"), Command::Help);
        assert_eq!(parse("inspect"), Command::Prompt("inspect".into()));
        assert_eq!(parse(&"a".repeat(INPUT_LIMIT + 1)), Command::TooLong);
    }

    #[test]
    fn renderer_hides_reasoning_and_does_not_duplicate_text() {
        let mut render = Renderer::default();
        let mut out = Vec::new();
        for (seq, event) in [
            AgentEvent::ThinkingDelta { delta: "private thinking".into() },
            AgentEvent::TextDelta { delta: "visible answer".into() },
        ].into_iter().enumerate() {
            assert!(!render.event(SessionEvent::TurnEvent {
                session_id: "s".into(), turn_id: "t".into(), sequence: seq as u64,
                payload: TurnEventPayload::Progress { event },
            }, &mut out).unwrap());
        }
        assert!(render.event(SessionEvent::TurnEvent {
            session_id: "s".into(), turn_id: "t".into(), sequence: 2,
            payload: TurnEventPayload::Completed { status: TurnCompletionStatus::Success {
                final_text: "visible answer".into(), new_messages: vec![]
            }},
        }, &mut out).unwrap());
        let output = String::from_utf8(out).unwrap();
        assert_eq!(output.matches("visible answer").count(), 1);
        assert!(!output.contains("private thinking"));
        assert!(output.contains("[turn:ok]"));
    }
}
