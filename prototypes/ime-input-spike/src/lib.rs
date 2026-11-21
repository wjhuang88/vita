//! Disposable, non-root IME key-to-text experiment. No model, device access or persistence.
use inputx_pinyin::{PinyinEngine, Session};
use unicode_segmentation::UnicodeSegmentation;

const PAGE_SIZE: usize = 5;
const MAX_PREEDIT: usize = 32;
const MAX_TEXT_BYTES: usize = 8192;
const MAX_PASTE_CHARS: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Chinese,
    English,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Input {
    Char(char),
    Space,
    Backspace,
    Delete,
    Left,
    Right,
    Enter,
    /// Deliberately distinct from Enter: legacy PTYs cannot prove key release.
    Submit,
    Escape,
    ToggleMode,
    NextPage,
    PreviousPage,
    Paste(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Outcome {
    Updated,
    Ignored,
    /// This is a *user-submitted* line; composing or selecting a candidate
    /// never submits the text to the Agent or any tool.
    Submitted(String),
}

#[derive(Debug)]
pub struct View {
    pub text: String,
    pub cursor_prefix: String,
    pub composing: String,
    pub mode: Mode,
    pub candidates: Vec<(usize, String)>,
    pub page: usize,
    pub pages: usize,
}

pub struct Editor<'e> {
    session: Session<'e>,
    mode: Mode,
    text: String,
    /// Unicode grapheme-cluster index, not bytes or terminal cells.
    cursor: usize,
    page: usize,
}

impl<'e> Editor<'e> {
    pub fn new(engine: &'e PinyinEngine) -> Self {
        Self {
            session: Session::new(engine),
            mode: Mode::Chinese,
            text: String::new(),
            cursor: 0,
            page: 0,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn composing(&self) -> &str {
        self.session.input()
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn view(&mut self) -> View {
        // Capture fields before borrowing the mutable candidate cache.
        let cursor_prefix = self.text[..self.byte_at_cursor()].to_owned();
        let composing = self.session.input().to_owned();
        let candidates = self.session.candidates();
        let pages = candidates.len().div_ceil(PAGE_SIZE);
        let start = self.page.saturating_mul(PAGE_SIZE);
        View {
            text: self.text.clone(),
            cursor_prefix,
            composing,
            mode: self.mode,
            candidates: candidates
                .iter()
                .skip(start)
                .take(PAGE_SIZE)
                .enumerate()
                .map(|(i, s)| (i + 1, s.clone()))
                .collect(),
            page: self.page,
            pages,
        }
    }

    fn byte_at_cursor(&self) -> usize {
        self.text
            .grapheme_indices(true)
            .nth(self.cursor)
            .map_or(self.text.len(), |(pos, _)| pos)
    }

    fn insert(&mut self, value: &str) {
        let available = MAX_TEXT_BYTES.saturating_sub(self.text.len());
        if available == 0 {
            return;
        }
        let mut clean = String::new();
        for c in value.chars() {
            // Prevent injected escape sequences, C0/C1 and bidi direction
            // overrides from reaching terminal output via the editor.
            if c.is_control() || is_bidi_control(c) {
                continue;
            }
            if clean.len() + c.len_utf8() > available {
                break;
            }
            clean.push(c);
        }
        let at = self.byte_at_cursor();
        self.text.insert_str(at, &clean);
        let byte_after = at + clean.len();
        self.cursor = self.text[..byte_after].graphemes(true).count();
    }

    fn backspace(&mut self) -> bool {
        if self.cursor == 0 {
            return false;
        }
        let from = self
            .text
            .grapheme_indices(true)
            .nth(self.cursor - 1)
            .map_or(0, |(pos, _)| pos);
        let to = self.byte_at_cursor();
        self.text.replace_range(from..to, "");
        self.cursor -= 1;
        true
    }

    fn delete(&mut self) -> bool {
        let from = self.byte_at_cursor();
        if from == self.text.len() {
            return false;
        }
        let to = self
            .text
            .grapheme_indices(true)
            .nth(self.cursor + 1)
            .map_or(self.text.len(), |(pos, _)| pos);
        self.text.replace_range(from..to, "");
        true
    }

    fn flush_raw_preedit(&mut self) {
        let raw = self.session.input().to_owned();
        self.session.reset();
        self.page = 0;
        self.insert(&raw);
    }

    fn commit_candidate(&mut self, idx: usize) -> bool {
        let chosen = self.session.commit(idx);
        if let Some(chosen) = chosen {
            self.page = 0;
            self.insert(&chosen);
            true
        } else {
            false
        }
    }

    fn commit_first_or_raw(&mut self) {
        if !self.commit_candidate(self.page * PAGE_SIZE) {
            self.flush_raw_preedit();
        }
    }

    pub fn handle(&mut self, key: Input) -> Outcome {
        let active = !self.session.input().is_empty();
        match key {
            Input::ToggleMode => {
                // Never implicitly accept an IME selection on mode change.
                if active {
                    self.flush_raw_preedit();
                }
                self.mode = match self.mode {
                    Mode::Chinese => Mode::English,
                    Mode::English => Mode::Chinese,
                };
            }
            Input::Paste(value) => {
                // The terminal's IME already committed Unicode; do not
                // feed it into our Pinyin engine a second time.
                if active {
                    self.flush_raw_preedit();
                }
                let pasted: String = value
                    .chars()
                    .take(MAX_PASTE_CHARS)
                    .map(|c| {
                        if c == '\n' || c == '\r' || c == '\t' {
                            ' '
                        } else {
                            c
                        }
                    })
                    .collect();
                self.insert(&pasted);
            }
            Input::Escape => {
                if !active {
                    return Outcome::Ignored;
                }
                self.session.reset();
                self.page = 0;
            }
            Input::Backspace => {
                if active {
                    self.session.backspace();
                    self.page = 0;
                } else if !self.backspace() {
                    return Outcome::Ignored;
                }
            }
            Input::Delete => {
                if active || !self.delete() {
                    return Outcome::Ignored;
                }
            }
            Input::Left => {
                if active || self.cursor == 0 {
                    return Outcome::Ignored;
                }
                self.cursor -= 1;
            }
            Input::Right => {
                if active || self.cursor >= self.text.graphemes(true).count() {
                    return Outcome::Ignored;
                }
                self.cursor += 1;
            }
            Input::NextPage | Input::PreviousPage => {
                if !active {
                    return Outcome::Ignored;
                }
                let pages = self.session.candidates().len().div_ceil(PAGE_SIZE);
                self.page = match key {
                    Input::NextPage if self.page + 1 < pages => self.page + 1,
                    Input::PreviousPage => self.page.saturating_sub(1),
                    _ => self.page,
                };
            }
            Input::Space => {
                if active {
                    // Space chooses first candidate, but must not submit line.
                    self.commit_first_or_raw();
                } else {
                    self.insert(" ");
                }
            }
            Input::Enter => {
                // No regular Enter can submit: PTYs without key-release events
                // cannot distinguish repeat of candidate confirmation.
                if active {
                    self.commit_first_or_raw();
                } else {
                    return Outcome::Ignored;
                }
            }
            Input::Submit => {
                if active || self.text.is_empty() {
                    return Outcome::Ignored;
                }
                self.cursor = 0;
                return Outcome::Submitted(std::mem::take(&mut self.text));
            }
            Input::Char(c) => {
                if self.mode == Mode::Chinese && c.is_ascii_alphabetic() {
                    if self.session.input().len() < MAX_PREEDIT {
                        self.session.input_char(c.to_ascii_lowercase());
                        self.page = 0;
                    } else {
                        return Outcome::Ignored;
                    }
                } else if active && ('1'..='5').contains(&c) {
                    let idx = self.page * PAGE_SIZE + (c as usize - '1' as usize);
                    if !self.commit_candidate(idx) {
                        return Outcome::Ignored;
                    }
                } else {
                    if active {
                        self.commit_first_or_raw();
                    }
                    let mut text = String::new();
                    text.push(c);
                    self.insert(&text);
                }
            }
        }
        Outcome::Updated
    }
}

fn is_bidi_control(c: char) -> bool {
    matches!(c,
        '\u{061c}' | '\u{200e}' | '\u{200f}' |
        '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn type_keys(e: &mut Editor<'_>, word: &str) {
        for c in word.chars() {
            assert_eq!(e.handle(Input::Char(c)), Outcome::Updated);
        }
    }

    #[test]
    fn pinyin_commits_separately_from_agent_submit() {
        let engine = PinyinEngine::new();
        let mut e = Editor::new(&engine);
        type_keys(&mut e, "zhongguo");
        assert_eq!(e.composing(), "zhongguo");
        assert_eq!(
            e.view().candidates.first().map(|(_, s)| s.as_str()),
            Some("中国")
        );
        assert_eq!(e.handle(Input::Enter), Outcome::Updated);
        assert_eq!(e.text(), "中国");
        assert!(e.composing().is_empty());
        assert_eq!(e.handle(Input::Enter), Outcome::Ignored);
        assert_eq!(e.handle(Input::Enter), Outcome::Ignored); // repeat is safe
        assert_eq!(e.handle(Input::Submit), Outcome::Submitted("中国".into()));
    }

    #[test]
    fn selection_and_cancel_do_not_leak_preedit_to_agent() {
        let engine = PinyinEngine::new();
        let mut e = Editor::new(&engine);
        type_keys(&mut e, "zhongguo");
        assert_eq!(e.handle(Input::Char('1')), Outcome::Updated);
        assert_eq!(e.text(), "中国");
        type_keys(&mut e, "nihao");
        assert_eq!(e.handle(Input::Escape), Outcome::Updated);
        assert_eq!(e.text(), "中国");
        assert!(e.composing().is_empty());
        assert_eq!(e.handle(Input::Enter), Outcome::Ignored);
        assert_eq!(e.handle(Input::Enter), Outcome::Ignored); // repeat is safe
        assert_eq!(e.handle(Input::Submit), Outcome::Submitted("中国".into()));
    }

    #[test]
    fn preedit_backspace_and_mode_switch_are_explicit() {
        let engine = PinyinEngine::new();
        let mut e = Editor::new(&engine);
        type_keys(&mut e, "ni");
        assert_eq!(e.handle(Input::Backspace), Outcome::Updated);
        assert_eq!(e.composing(), "n");
        assert_eq!(e.handle(Input::ToggleMode), Outcome::Updated);
        assert_eq!(e.mode(), Mode::English);
        assert_eq!(e.text(), "n"); // raw pinyin preserved, never auto-selected
        type_keys(&mut e, "hi");
        assert_eq!(e.text(), "nhi");
    }

    #[test]
    fn external_utf8_paste_is_never_reinterpreted_as_pinyin() {
        let engine = PinyinEngine::new();
        let mut e = Editor::new(&engine);
        assert_eq!(
            e.handle(Input::Paste("你好, Vita 👩‍💻".into())),
            Outcome::Updated
        );
        assert_eq!(e.text(), "你好, Vita 👩‍💻");
        assert_eq!(e.handle(Input::Backspace), Outcome::Updated);
        assert_eq!(e.text(), "你好, Vita ");
        assert_eq!(e.handle(Input::Left), Outcome::Updated);
        assert_eq!(e.handle(Input::Char('!')), Outcome::Updated);
        assert_eq!(e.text(), "你好, Vita! ");
    }

    #[test]
    fn strips_terminal_control_and_direction_override_from_paste() {
        let engine = PinyinEngine::new();
        let mut e = Editor::new(&engine);
        e.handle(Input::Paste("\x1b]52;secret\x07你\n好\u{202e}A".into()));
        assert_eq!(e.text(), "]52;secret你 好A");
        // Escape/control bytes are absent, even if literal printable text remains.
        assert!(
            !e.text()
                .chars()
                .any(|c| c.is_control() || is_bidi_control(c))
        );
    }

    #[test]
    fn bounded_composition_paste_and_editor() {
        let engine = PinyinEngine::new();
        let mut e = Editor::new(&engine);
        type_keys(&mut e, &"a".repeat(MAX_PREEDIT));
        assert_eq!(e.handle(Input::Char('a')), Outcome::Ignored);
        assert_eq!(e.composing().len(), MAX_PREEDIT);
        assert_eq!(e.handle(Input::Escape), Outcome::Updated);
        e.handle(Input::Paste("中".repeat(MAX_PASTE_CHARS + 10)));
        assert!(e.text().len() <= MAX_TEXT_BYTES);
        assert_eq!(e.text().len(), MAX_TEXT_BYTES - MAX_TEXT_BYTES % "中".len());
    }

    #[test]
    fn cursor_grapheme_delete_respects_combining_and_emoji() {
        let engine = PinyinEngine::new();
        let mut e = Editor::new(&engine);
        e.handle(Input::Paste("e\u{0301}👩‍💻中文".into()));
        assert_eq!(e.handle(Input::Backspace), Outcome::Updated);
        assert_eq!(e.text(), "e\u{0301}👩‍💻中");
        assert_eq!(e.handle(Input::Backspace), Outcome::Updated);
        assert_eq!(e.handle(Input::Backspace), Outcome::Updated);
        assert_eq!(e.text(), "e\u{0301}");
        assert_eq!(e.handle(Input::Backspace), Outcome::Updated);
        assert!(e.text().is_empty());
    }

    #[test]
    fn paging_and_empty_submission_are_bounded() {
        let engine = PinyinEngine::new();
        let mut e = Editor::new(&engine);
        assert_eq!(e.handle(Input::Enter), Outcome::Ignored);
        assert_eq!(e.handle(Input::Submit), Outcome::Ignored);
        assert_eq!(e.handle(Input::NextPage), Outcome::Ignored);
        type_keys(&mut e, "shi");
        let before = e.view().page;
        e.handle(Input::NextPage);
        assert!(e.view().page >= before);
        e.handle(Input::PreviousPage);
        assert_eq!(e.view().page, 0);
    }
    #[test]
    fn page_two_confirmation_uses_visible_candidate_for_space_and_enter() {
        let engine = PinyinEngine::new();
        for confirm in [Input::Space, Input::Enter] {
            let mut e = Editor::new(&engine);
            type_keys(&mut e, "shi");
            e.handle(Input::NextPage);
            let page = e.view();
            assert!(page.page >= 1, "shi must have multiple candidate pages");
            let expected = page
                .candidates
                .first()
                .expect("visible candidate")
                .1
                .clone();
            assert_eq!(e.handle(confirm), Outcome::Updated);
            assert_eq!(e.text(), expected);
            assert!(e.composing().is_empty());
            assert_eq!(e.handle(Input::Submit), Outcome::Submitted(expected));
        }
    }

    #[test]
    fn secret_field_simulated_submit_requires_distinct_event() {
        let engine = PinyinEngine::new();
        let mut e = Editor::new(&engine);
        type_keys(&mut e, "nihao");
        assert_eq!(e.handle(Input::Submit), Outcome::Ignored);
        e.handle(Input::Enter);
        assert_eq!(e.handle(Input::Enter), Outcome::Ignored);
        assert!(!e.text().is_empty());
        assert!(matches!(e.handle(Input::Submit), Outcome::Submitted(_)));
    }
}
