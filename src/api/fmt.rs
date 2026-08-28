// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later OR GPL-3.0-or-later

//! Text formatting API
//! ====================================================================
//!
//! This provides methods to manipulate with text representation, like
//! text wrapping.

use crate::terminal::{
    NeverTerminal,
    terminal_get_size
};

use std::num::NonZeroU16 as u16_nz;

#[cfg(feature = "unicode")]
use unicode_width::*;

/// Trait for terminal-focused formatting
/// ====================================================================
///
/// It extends string-like types to enable them with terminal formatting
/// capabilities.
///
pub trait TtyFormat:AsRef<str> {
    /// Normalizes tabulation width by converting it to space character
    /// representation.
    #[cfg(feature = "unicode")]
    fn tabs_as_spaces(&self, tab_width:u8)->String {
        let mut pos = 0;
        self.as_ref()
            .chars()
            .map(|c| {
                let res = match c {
                    '\t' => " ".repeat((tab_width-(pos%tab_width)) as usize),
                    '\n' => { pos = u8::MAX; c.to_string() },
                    _ => c.to_string()
                };
                pos = pos.overflowing_add(c.width().unwrap_or(0) as u8).0;
                res
            })
            .collect::<String>()
    }
    /// Like `term_wrap`, but specialized under the asumptions you will
    /// operate on ASCII character set. This allows for optimizations
    /// under the assumptions that strings take as many collumns as
    /// they take bytes, which makes it easier to seek as arbitrary
    /// jumps can be made than to seek with length-counting.
    #[cfg(feature = "ascii")]
    fn ascii_term_wrap(&self, cols:Option<u16_nz>, ind:u16)->String {
        // Text reference
        let text = self.as_ref();
        // Terminal width
        let width = cols.or_else(|| terminal_get_size::<NeverTerminal>(None)
            .map(|p| p.0));
        if let Some(width) = width {
            // Line limit
            let wlimit = (width.get()-ind) as usize;
            // Output string
            let mut result = String::with_capacity(text.len());
            // Indentation
            let ind = " ".repeat(ind as usize).into_boxed_str();
            for mut line in text.split('\n') { loop {
                line = line.trim_start();
                // Case 1: Line within bounds
                if line.len() <= wlimit {
                    result+=(ind.to_string()+line+"\n").as_str();
                    break;
                }
                // Case 2: Word split
                let last_idx = line.as_bytes()[..wlimit].iter()
                    .rposition(u8::is_ascii_whitespace);
                if let Some(idx) = last_idx {
                    let lines = line.split_at(idx);
                    result+=(ind.to_string()+lines.0+"\n").as_str();
                    line = lines.1;
                } else {
                    // Case 3: Lengthy words (no wrap)
                    result+=(ind.to_string()+line+"\n").as_str();
                    break;
                }
            }}
            result
        } else {
            // Fallback
            text.to_string()
        }
    }
    /// Wraps the text to `cols`, placing words after `ind` spaces and
    /// by replacing whitespaces between words with space. It does not
    /// split words.
    #[cfg(feature = "unicode")]
    fn term_wrap(&self, cols:Option<u16_nz>, ind:u16)->String {
        // Text reference
        let text = self.as_ref();
        // Terminal width
        let width = cols.or_else(|| terminal_get_size::<NeverTerminal>(None)
            .map(|p| p.0));
        if let Some(width) = width {
            // Line limit
            let wlimit = (width.get()-ind) as usize;
            // Output string
            let mut result = String::with_capacity(text.len());
            // Indentation
            let ind = " ".repeat(ind as usize).into_boxed_str();
            for mut line in text.split('\n') { loop {
                line = line.trim_start();
                // Case 1: Line within bounds
                if line.width() <= wlimit {
                    result+=(ind.to_string()+line+"\n").as_str();
                    break;
                }
                // Case 2: Word split
                let last_idx = {
                    let mut w_sum = 0usize;
                    let bound = line.find(|c:char| {w_sum+=c.width().unwrap_or(0);w_sum>=wlimit}).unwrap_or(line.len());
                    line[..bound].rfind(char::is_whitespace)
                };
                if let Some(idx) = last_idx {
                    let lines = line.split_at(idx);
                    result+=(ind.to_string()+lines.0+"\n").as_str();
                    line = lines.1;
                } else {
                    // Case 3: Lengthy words (no wrap)
                    result+=(ind.to_string()+line+"\n").as_str();
                    break;
                }
            }}
            result
        } else {
            // Fallback
            text.to_string()
        }
    }
}

impl<T:AsRef<str>> TtyFormat for T {}
