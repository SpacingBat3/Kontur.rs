// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Terminal text styling API
//! ====================================================================
//!
//! A macro and enum based API inspired by Node.js `util.styleText`, to
//! apply custom text styles.
//!
//! Note: the API is still experimental and its implementation might
//! drastically change.

use std::hint::unreachable_unchecked;

pub(crate) mod attrib;
pub(crate) mod color;

pub use attrib::*;
pub use color::*;

/// A type describing atomic ANSI sequence escape for styling text.
pub enum Style {
    /// Color definition
    Color(Color),
    /// Any known attribute
    Attribute(Attribute)
}

impl Style {
    /// Fetches a record representation of type, i.e. something like
    /// ([u8],[u8]), where `[u8]` represents the array with numeric
    /// control sequence representation.
    pub(crate) fn to_record(&self)->(Box<[u8]>,Box<[u8]>) {
        let mut left = Vec::with_capacity(5); match self {
            Self::Color(Color::FgPallete(p)) => { left.push(*p as u8); },
            Self::Color(Color::BgPallete(p)) => { left.push((*p as u8)+10); },
            Self::Color(c) => {
                left.push(if c.is_fg() { 38 } else { 48 });
                match c {
                    Color::FgPallete(_) | Color::BgPallete(_)
                        => unsafe { unreachable_unchecked() }
                    Color::FgByte(n) | Color::BgByte(n)
                        => left.extend([5,*n]),
                    Color::FgRGB(r,g,b) | Color::BgRGB(r,g,b)
                        => left.extend([2,*r,*g,*b]),
                };
            },
            Self::Attribute(a) => left.push(*a as u8)
        };
        let right:u8 = match self {
            Self::Color(c) => if c.is_fg() { 39 } else { 49 },
            Self::Attribute(a) => match a {
                Attribute::Reset => *a as u8,
                // Intensity
                Attribute::Bold | Attribute::Dim => 22,
                // Common attribute offsets
                Attribute::Italic | Attribute::Underline |
                Attribute::Blink | Attribute::Invert | Attribute::Hide |
                Attribute::Strikethrough => 20+(*a as u8),
                Attribute::Frame | Attribute::Overline => 3+(*a as u8),
                // Shared with previous
                Attribute::RapidBlink => 20+(Attribute::Blink as u8),
                Attribute::DoubleUnderline => 20+(Attribute::Underline as u8),
                Attribute::Encircle => 3+(Attribute::Frame as u8)
            }
        };
        (left.into_boxed_slice(),Box::new([right]))
    }
    /// Like `to_record`, but as `String` than value chain.
    pub fn to_string_record(&self)->[String;2] {
        let (s,e) = self.to_record();
        [s,e].map(|v| v
            .iter()
            .map(u8::to_string)
            .collect::<Vec<String>>()
            .join(";")
        )
    }
    /// Applies style to given `text`.
    /// ================================================================
    ///
    /// You might chain invocations and get expected results, e.g.
    /// `red.apply(format!("{}bar",blue.apply("foo")))` (conceptually)
    /// will result in string with blue `foo` and red `bar`.
    pub fn apply<T:AsRef<str>>(&self,text:T)->String {
        // FIXME: detect color support on use
        let [start,end] = self.to_string_record();
        format!("\x1b[{start}m{}\x1b[{end}m",text.as_ref().replace(
            format!("\x1b[{start}m").as_str(),
            format!("\x1b[{end}m").as_str()
        ))
    }
    /// Guards text against further style leaks of the same type.
    /// ================================================================
    ///
    /// This places a control sequence in given text position, so you
    /// might arbitarly disable style rendering from a given point.
    ///
    pub fn barrier<T:AsRef<str>>(&self,text:T,pos:usize)->String {
        let [_,end] = self.to_string_record();
        let mut text = text.as_ref().to_string();
        text.insert_str(pos, format!("\x1b{}m",end).as_str());
        text
    }
}
