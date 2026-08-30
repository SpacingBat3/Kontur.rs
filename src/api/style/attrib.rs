// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Attribute definitions
//! ====================================================================
//! 
//! 

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// Represents ANSI attributes known by this crate.
pub enum Attribute {
    /// Entirely resets all styling to the default
    Reset,
    Bold,
    Dim,
    Italic=3,
    Underline,
    Blink,
    RapidBlink,
    Invert,
    Hide,
    Strikethrough,
    DoubleUnderline=21,
    Frame=51,
    Encircle,
    Overline
}
