//! A standalone, offline PTY TUI; does not embed or operate Vita/Talos.
use std::{
    io,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
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
use signal_hook::{
    consts::signal::{SIGHUP, SIGINT, SIGTERM},
    flag,
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;
use vita_ime_input_spike::{Editor, Input, Mode, Outcome, View};

/// Bracketed paste stays disabled intentionally. Crossterm's Event::Paste
/// accumulates an unbounded payload before the application can reject it.
/// Normal UTF-8 key events remain usable for externally committed text.
/// A bounded *streaming decoder* is a future requirement before enabling
/// bracketed paste in a shipping TUI.
struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        if let Err(err) = execute!(io::stdout(), EnterAlternateScreen) {
            let _ = disable_raw_mode();
            return Err(err);
        }
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

fn key_to_input(key: KeyEvent) -> Option<Input> {
    // Explicit submission gesture cannot be confused with repeating Enter
    // on traditional PTYs that do not report key release events.
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('d')) {
        return Some(Input::Submit);
    }
    if key
        .modifiers
        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SUPER)
    {
        if key.modifiers.contains(KeyModifiers::CONTROL)
            && matches!(key.code, KeyCode::Char(' ') | KeyCode::Null)
        {
            return Some(Input::ToggleMode);
        }
        return None;
    }
    match key.code {
        KeyCode::F(2) => Some(Input::ToggleMode),
        KeyCode::F(4) => Some(Input::Submit),
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

/// Return a one-line cursor-following viewport. The input's grapheme cursor
/// is converted to *terminal cells* and clamped after a PTY resize.
fn viewport(text: &str, cursor_prefix: &str, columns: usize) -> (String, usize) {
    if columns == 0 {
        return (String::new(), 0);
    }
    let limit = columns.saturating_sub(1);
    let mut left = Vec::new();
    let mut cells = 0;
    for g in cursor_prefix.graphemes(true).rev() {
        let w = g.width();
        if cells + w > limit {
            break;
        }
        cells += w;
        left.push(g);
    }
    left.reverse();
    let mut window = left.concat();
    let cursor_cell = cells;
    for g in text[cursor_prefix.len()..].graphemes(true) {
        let w = g.width();
        if cells + w > limit {
            break;
        }
        cells += w;
        window.push_str(g);
    }
    (window, cursor_cell)
}

fn tail_in_cells(text: &str, columns: usize) -> String {
    let mut visible = Vec::new();
    let mut cells = 0;
    for g in text.graphemes(true).rev() {
        let w = g.width();
        if cells + w > columns {
            break;
        }
        cells += w;
        visible.push(g);
    }
    visible.reverse();
    visible.concat()
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
            "S={} · Vita IME PTY spike · {mode} · F2 switch · Ctrl+D/F4 submit · Ctrl+C exit",
            submitted.len()
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Offline input only"),
        ),
        sections[0],
    );

    // Reserve a distinct preedit row, and never let an 8 KiB editor paragraph
    // wrap the cursor beyond the viewport. All widths are terminal *cells*.
    let label = "文本: ";
    let preedit_label = "拼音: ";
    let text_columns = (sections[1].width as usize).saturating_sub(2 + label.width());
    let preedit_columns = (sections[1].width as usize).saturating_sub(2 + preedit_label.width());
    let (text_window, cursor_cell) = viewport(&view.text, &view.cursor_prefix, text_columns);
    let preedit_window = tail_in_cells(&view.composing, preedit_columns);
    frame.render_widget(
        Paragraph::new(format!(
            "{label}{text_window}\n{preedit_label}{preedit_window}"
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Editor / preedit"),
        ),
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
            "{page}: {candidates}\n1-5/Space/Enter: choose · Tab: page · Esc: cancel"
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("IME candidates"),
        )
        .wrap(Wrap { trim: false }),
        sections[2],
    );

    let lines = if submitted.is_empty() {
        "(none; Enter cannot submit)".to_owned()
    } else {
        submitted
            .iter()
            .rev()
            .take(8)
            .rev()
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    };
    frame.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Submitted lines"),
        ),
        sections[3],
    );

    if sections[1].width > label.width() as u16 + 2 && sections[1].height >= 3 {
        let x = sections[1]
            .x
            .saturating_add(1 + label.width() as u16 + cursor_cell as u16)
            .min(sections[1].right().saturating_sub(2));
        frame.set_cursor_position((x, sections[1].y + 1));
    }
}

fn run_tui() -> io::Result<()> {
    let stop = Arc::new(AtomicBool::new(false));
    for signal in [SIGTERM, SIGINT, SIGHUP] {
        flag::register(signal, Arc::clone(&stop))?;
    }
    let _guard = TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let engine = PinyinEngine::new();
    let mut editor = Editor::new(&engine);
    let mut submitted = Vec::new();

    while !stop.load(Ordering::Relaxed) {
        let view = editor.view();
        terminal.draw(|frame| draw(frame, &view, &submitted))?;
        if !event::poll(Duration::from_millis(100))? {
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
                    // The list is a local demonstration, no Tool/Agent call.
                    submitted.push(line);
                    if submitted.len() > 8 {
                        submitted.remove(0);
                    }
                }
            }
            Event::Resize(_, _) => terminal.autoresize()?,
            // No Event::Paste: upstream's decoder has no pre-allocation cap.
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
        return Err("Enter must choose a candidate without submitting");
    }
    if e.handle(Input::Enter) != Outcome::Ignored
        || e.handle(Input::Submit) != Outcome::Submitted("中国".into())
    {
        return Err("Ctrl+D/F4 is the only text submission path");
    }
    println!("VITA_IME_SELFTEST_OK pinyin=zhongguo commit=中国 submit=explicit");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_keeps_long_wide_unicode_cursor_visible() {
        let text = format!("{}中文👩‍💻", "a".repeat(80));
        let (view, x) = viewport(&text, &text, 18);
        assert!(x < 18);
        assert!(view.ends_with("中文👩‍💻"), "{view}");
        assert!(view.width() <= 18);
    }

    #[test]
    fn viewport_clamps_after_small_resize_and_keeps_preedit_row() {
        let text = "中文abcdefghijklmnopqrstuvwxyz";
        for columns in [0, 1, 5, 8, 40] {
            let (visible, x) = viewport(text, text, columns);
            assert!(visible.width() <= columns);
            assert!(columns == 0 || x < columns);
        }
        assert_eq!(tail_in_cells("拼音zhongguo", 5), "gguo");
    }
}
