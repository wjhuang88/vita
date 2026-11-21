//! Small bounded byte-stream decoder for the offline PTY experiment.
//!
//! Crucially this path NEVER uses crossterm::event::{poll, read}. Crossterm's
//! default-enabled bracketed-paste parser can buffer without a length cap.
//! We explicitly discard *all* unsolicited bracketed-paste contents, bound
//! escape sequences and UTF-8 state, and always recognize Ctrl+C even inside
//! malformed or unfinished paste. This is not a general-purpose VT parser.
use std::time::{Duration, Instant};

use vita_ime_input_spike::Input;

const ESC: u8 = 0x1b;
const ESC_LIMIT: usize = 16;
const ESC_TIMEOUT: Duration = Duration::from_millis(80);
const PASTE_TIMEOUT: Duration = Duration::from_secs(2);
const PASTE_END: &[u8] = b"\x1b[201~";

#[derive(Debug, PartialEq)]
pub enum Decoded {
    Editor(Input),
    Exit,
}

#[derive(Default)]
pub struct Decoder {
    escape: Vec<u8>,
    esc_since: Option<Instant>,
    discard_csi_since: Option<Instant>,
    in_paste: bool,
    paste_since: Option<Instant>,
    paste_tail: Vec<u8>,
    utf8: Vec<u8>,
}

impl Decoder {
    pub fn new() -> Self {
        Self::default()
    }

    fn reset_escape(&mut self) {
        self.escape.clear();
        self.esc_since = None;
    }

    fn reset_paste(&mut self) {
        self.in_paste = false;
        self.paste_since = None;
        self.paste_tail.clear();
    }

    /// Called even if no bytes arrive. An incomplete escape becomes one Esc
    /// key after a short timeout, while a partial paste is dropped entirely.
    pub fn tick(&mut self) -> Vec<Decoded> {
        let mut out = Vec::new();
        if self
            .esc_since
            .is_some_and(|start| start.elapsed() >= ESC_TIMEOUT)
        {
            self.reset_escape();
            out.push(Decoded::Editor(Input::Escape));
        }
        if self
            .discard_csi_since
            .is_some_and(|start| start.elapsed() >= ESC_TIMEOUT)
        {
            self.discard_csi_since = None;
        }
        if self
            .paste_since
            .is_some_and(|start| start.elapsed() >= PASTE_TIMEOUT)
        {
            self.reset_paste();
        }
        out
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Vec<Decoded> {
        let mut result = Vec::new();
        for &byte in bytes {
            // Safety invariant: Ctrl+C is never swallowed by an incomplete
            // ESC prefix, unsolicited paste or malformed UTF-8.
            if byte == 0x03 {
                self.reset_escape();
                self.reset_paste();
                self.utf8.clear();
                result.push(Decoded::Exit);
                continue;
            }
            // Bound lifetime when input continues but the closing marker is
            // absent. No paste content is retained or sent to the editor.
            if self.in_paste {
                if self
                    .paste_since
                    .is_some_and(|start| start.elapsed() >= PASTE_TIMEOUT)
                {
                    self.reset_paste();
                } else {
                    self.paste_tail.push(byte);
                    if self.paste_tail.len() > PASTE_END.len() {
                        self.paste_tail.remove(0);
                    }
                    if self.paste_tail == PASTE_END {
                        self.reset_paste();
                    }
                    continue;
                }
            }

            // Reject the complete unknown CSI sequence (including all
            // parameters), not just its prefix. Never turn its trailing
            // digits into IME text or an accidental submission.
            if self.discard_csi_since.is_some() {
                if (0x40..=0x7e).contains(&byte) {
                    self.discard_csi_since = None;
                }
                continue;
            }
            if !self.escape.is_empty() {
                self.escape.push(byte);
                self.parse_escape(&mut result);
                continue;
            }
            if byte == ESC {
                self.escape.push(ESC);
                self.esc_since = Some(Instant::now());
                continue;
            }
            self.parse_text(byte, &mut result);
        }
        result
    }

    fn parse_escape(&mut self, out: &mut Vec<Decoded>) {
        let known: &[(&[u8], Option<Input>)] = &[
            (b"\x1b[200~", None),                 // enter discard-only paste
            (b"\x1b[201~", None),                 // stray end marker
            (b"\x1bOQ", Some(Input::ToggleMode)), // F2
            (b"\x1bOS", Some(Input::Submit)),     // F4
            (b"\x1b[12~", Some(Input::ToggleMode)),
            (b"\x1b[14~", Some(Input::Submit)),
            (b"\x1b[D", Some(Input::Left)),
            (b"\x1b[C", Some(Input::Right)),
            (b"\x1b[5~", Some(Input::PreviousPage)),
            (b"\x1b[6~", Some(Input::NextPage)),
            (b"\x1b[Z", Some(Input::PreviousPage)),
        ];
        if self.escape.as_slice() == b"\x1b[200~" {
            self.reset_escape();
            self.in_paste = true;
            self.paste_since = Some(Instant::now());
            return;
        }
        for (sequence, result) in known {
            if sequence == &self.escape.as_slice() {
                if let Some(key) = result {
                    // Input enum is not Clone, so match the specific known
                    // variants; no owned arbitrary terminal payload occurs.
                    let input = match key {
                        Input::ToggleMode => Input::ToggleMode,
                        Input::Submit => Input::Submit,
                        Input::Left => Input::Left,
                        Input::Right => Input::Right,
                        Input::PreviousPage => Input::PreviousPage,
                        Input::NextPage => Input::NextPage,
                        _ => return,
                    };
                    out.push(Decoded::Editor(input));
                }
                self.reset_escape();
                return;
            }
        }
        // Unknown sequences are dropped, never passed as printable text.
        // The fixed buffer and idle timeout prevent unbounded growth.
        if self.escape.len() >= ESC_LIMIT
            || !known
                .iter()
                .any(|(sequence, _)| sequence.starts_with(&self.escape))
        {
            // The remainder of an unsupported CSI must be dropped
            // through its final byte; otherwise e.g. ESC[199...~
            // becomes printable digits in the user editor.
            let csi = self.escape.starts_with(b"\x1b[");
            let final_seen = self
                .escape
                .last()
                .is_some_and(|byte| (0x40..=0x7e).contains(byte));
            if csi && !final_seen {
                self.discard_csi_since = Some(Instant::now());
            }
            self.reset_escape();
        }
    }

    fn parse_text(&mut self, byte: u8, out: &mut Vec<Decoded>) {
        if byte & 0x80 != 0 || !self.utf8.is_empty() {
            if self.utf8.len() >= 4 {
                self.utf8.clear();
            }
            self.utf8.push(byte);
            match std::str::from_utf8(&self.utf8) {
                Ok(text) => {
                    if let Some(ch) = text.chars().next() {
                        out.push(Decoded::Editor(Input::Char(ch)));
                    }
                    self.utf8.clear();
                }
                Err(err) if err.error_len().is_some() => self.utf8.clear(),
                Err(_) => {}
            }
            return;
        }
        let input = match byte {
            0x04 => Some(Input::Submit),
            0x00 => Some(Input::ToggleMode),
            0x08 | 0x7f => Some(Input::Backspace),
            b'\r' | b'\n' => Some(Input::Enter),
            b'\t' => Some(Input::NextPage),
            b' ' => Some(Input::Space),
            0x20..=0x7e => Some(Input::Char(char::from(byte))),
            _ => None,
        };
        if let Some(input) = input {
            out.push(Decoded::Editor(input));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragmented_utf8_and_explicit_submit_are_bounded() {
        let mut parser = Decoder::new();
        assert!(parser.feed(&[0xe4, 0xbd]).is_empty());
        assert_eq!(
            parser.feed(&[0xa0]),
            vec![Decoded::Editor(Input::Char('你'))]
        );
        assert_eq!(parser.feed(&[0x04]), vec![Decoded::Editor(Input::Submit)]);
        assert_eq!(
            parser.feed(b"\r\r"),
            vec![Decoded::Editor(Input::Enter), Decoded::Editor(Input::Enter)]
        );
    }

    #[test]
    fn unsolicited_incomplete_paste_never_swallows_ctrl_c() {
        let mut parser = Decoder::new();
        assert!(parser.feed(b"\x1b[200~").is_empty());
        assert!(parser.feed(&vec![b'A'; 128 * 1024]).is_empty());
        assert!(parser.paste_tail.len() <= PASTE_END.len());
        assert_eq!(parser.feed(&[0x03]), vec![Decoded::Exit]);
        assert!(!parser.in_paste);
        assert_eq!(parser.feed(b"x"), vec![Decoded::Editor(Input::Char('x'))]);
    }

    #[test]
    fn unsolicited_complete_paste_is_discarded() {
        let mut parser = Decoder::new();
        assert!(parser.feed(b"\x1b[200~secret\x1b[201~").is_empty());
        assert!(!parser.in_paste);
        assert_eq!(parser.feed(b"y"), vec![Decoded::Editor(Input::Char('y'))]);
    }

    #[test]
    fn handles_arrows_function_keys_and_bounded_unknown_escape() {
        let mut parser = Decoder::new();
        assert_eq!(parser.feed(b"\x1b[D"), vec![Decoded::Editor(Input::Left)]);
        assert_eq!(parser.feed(b"\x1bOS"), vec![Decoded::Editor(Input::Submit)]);
        assert_eq!(
            parser.feed(b"\x1bOQ"),
            vec![Decoded::Editor(Input::ToggleMode)]
        );
        assert!(parser.feed(b"\x1b[1999999999999999~").is_empty());
        assert!(parser.escape.is_empty());
        assert!(parser.discard_csi_since.is_none());
        assert_eq!(parser.feed(b"x"), vec![Decoded::Editor(Input::Char('x'))]);
    }

    #[test]
    fn split_unknown_csi_does_not_inject_ascii_or_ctrl_d() {
        let mut parser = Decoder::new();
        assert!(parser.feed(b"\x1b[199").is_empty());
        // Even Ctrl+D occurring as a literal in a malformed unfinished CSI
        // is consumed without ever returning an accidental submit.
        assert!(parser.feed(b"01234").is_empty());
        assert!(parser.feed(b"~").is_empty());
        assert_eq!(parser.feed(b"x"), vec![Decoded::Editor(Input::Char('x'))]);
    }

    #[test]
    fn timed_out_paste_does_not_trap_later_keys() {
        let mut parser = Decoder::new();
        parser.feed(b"\x1b[200~");
        parser.paste_since = Some(Instant::now() - PASTE_TIMEOUT);
        assert!(parser.tick().is_empty());
        assert!(!parser.in_paste);
        assert_eq!(parser.feed(b"z"), vec![Decoded::Editor(Input::Char('z'))]);
    }
}
