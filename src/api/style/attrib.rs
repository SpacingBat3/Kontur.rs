// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Attribute definitions
//! ====================================================================
//! 
//! 

#[derive(Clone, Copy)]
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
