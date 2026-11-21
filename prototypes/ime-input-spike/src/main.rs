//! A self-contained PTY/TUI frontend to validate IME behavior; not a Vita OS session.
use std::{io, time::Duration};

use crossterm::{
    event::{
        self, DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use inputx_pinyin::PinyinEngine;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use unicode_width::UnicodeWidthStr;
use vita_ime_input_spike::{Editor, Input, Mode, Outcome, View};

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        if let Err(err) = execute!(io::stdout(), EnterAlternateScreen, EnableBracketedPaste) {
            let _ = disable_raw_mode();
            return Err(err);
        }
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), DisableBracketedPaste, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

fn key_to_input(key: KeyEvent) -> Option<Input> {
    if key
        .modifiers
        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SUPER)
    {
        // Ctrl+Space may arrive as Char(' '), depending on the PTY/terminal.
        if key.modifiers.contains(KeyModifiers::CONTROL)
            && matches!(key.code, KeyCode::Char(' ') | KeyCode::Null)
        {
            return Some(Input::ToggleMode);
        }
        return None;
    }
    match key.code {
        KeyCode::F(2) => Some(Input::ToggleMode),
        KeyCode::Backspace => Some(Input::Backspace),
        KeyCode::Delete => Some(Input::Delete),
        KeyCode::Enter => Some(Input::Enter),
        KeyCode::Esc => Some(Input::Escape),
        KeyCode::Left => Some(Input::Left),
        KeyCode::Right => Some(Input::Right),
        KeyCode::Tab | KeyCode::PageDown => Some(Input::NextPage),
        KeyCode::BackTab | KeyCode::PageUp => Some(Input::PreviousPage),
        KeyCode::Char(' ') => Some(Input::Space),
        KeyCode::Char(c) => Some(Input::Char(c)),
        _ => None,
    }
}

fn draw(frame: &mut Frame<'_>, view: &View, submitted: &[String]) {
    let sections = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Min(3),
    ])
    .split(frame.area());

    let mode = match view.mode {
        Mode::Chinese => "中 / Pinyin",
        Mode::English => "EN / plain Unicode",
    };
    frame.render_widget(
        Paragraph::new(format!(
            "Vita IME PTY spike · {mode} · F2/Ctrl+Space switch · Ctrl+C exit"
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Offline input only"),
        ),
        sections[0],
    );

    frame.render_widget(
        Paragraph::new(format!("文本: {}\n拼音: {}", view.text, view.composing))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Editor / preedit"),
            )
            .wrap(Wrap { trim: false }),
        sections[1],
    );

    let candidates = view
        .candidates
        .iter()
        .map(|(i, text)| format!("{i}.{text}"))
        .collect::<Vec<_>>()
        .join("  ");
    let page = if view.pages == 0 {
        "no candidates".into()
    } else {
        format!("page {}/{}", view.page + 1, view.pages)
    };
    frame.render_widget(
        Paragraph::new(format!(
            "{page}: {candidates}\n1-5 select · Space/Enter commit · Tab page · Esc cancel"
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("IME candidates"),
        )
        .wrap(Wrap { trim: false }),
        sections[2],
    );

    frame.render_widget(
        Paragraph::new(
            submitted
                .iter()
                .rev()
                .take(8)
                .rev()
                .cloned()
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Submitted lines (no model/tool calls)"),
        )
        .wrap(Wrap { trim: false }),
        sections[3],
    );

    // Real cursor position, measured in terminal cells rather than UTF-8 bytes.
    let input_row = sections[1].y.saturating_add(1);
    let cell = "文本: ".width().saturating_add(view.cursor_prefix.width());
    if cell < sections[1].width.saturating_sub(2) as usize {
        frame.set_cursor_position((sections[1].x + 1 + cell as u16, input_row));
    }
}

fn run_tui() -> io::Result<()> {
    let _guard = TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let engine = PinyinEngine::new();
    let mut editor = Editor::new(&engine);
    let mut submitted = Vec::new();
    loop {
        let view = editor.view();
        terminal.draw(|frame| draw(frame, &view, &submitted))?;
        if !event::poll(Duration::from_millis(500))? {
            continue;
        }
        match event::read()? {
            Event::Key(key) if key.kind != KeyEventKind::Release => {
                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && matches!(key.code, KeyCode::Char('c'))
                {
                    break;
                }
                if let Some(input) = key_to_input(key)
                    && let Outcome::Submitted(line) = editor.handle(input)
                {
                    // Demo only: no agent operation is ever triggered by an IME commit.
                    submitted.push(line);
                    if submitted.len() > 8 {
                        submitted.remove(0);
                    }
                }
            }
            Event::Paste(text) => {
                editor.handle(Input::Paste(text));
            }
            Event::Resize(_, _) => terminal.autoresize()?,
            _ => {}
        }
    }
    Ok(())
}

fn run_self_test() -> Result<(), &'static str> {
    let engine = PinyinEngine::new();
    let mut e = Editor::new(&engine);
    for c in "zhongguo".chars() {
        e.handle(Input::Char(c));
    }
    if e.view().candidates.first().map(|(_, s)| s.as_str()) != Some("中国") {
        return Err("pinyin dictionary lookup failed");
    }
    if e.handle(Input::Enter) != Outcome::Updated || e.text() != "中国" {
        return Err("first Enter must select candidate without submitting");
    }
    if e.handle(Input::Enter) != Outcome::Submitted("中国".into()) {
        return Err("second Enter must submit committed text");
    }
    println!("VITA_IME_SELFTEST_OK pinyin=zhongguo commit=中国");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().skip(1).eq(["--self-test"]) {
        return run_self_test().map_err(Into::into);
    }
    if std::env::args().len() != 1 {
        return Err("usage: vita-ime-input-spike [--self-test]".into());
    }
    run_tui()?;
    println!("VITA_IME_TUI_EXIT_OK");
    Ok(())
}
