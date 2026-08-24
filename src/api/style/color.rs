// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Color definitions
//! ====================================================================
//!
//! This defines types that can describe various color systems in
//! context of terminals. ANSI sequences support is cross-platform,
//! albeit still dependant on application implementation.

/// An ANSI color name.
/// ====================================================================
///
/// Kontur currently approaches a more natural color pallete names,
/// defaulting to the light colors you might prefer to more frequently
/// use due to usually better contrast with the terminal background.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum AnsiColor {
    Black=30,
    #[doc(alias("Red"))]
    DarkRed,
    #[doc(alias("Green"))]
    DarkGreen,
    #[doc(alias("Yellow"))]
    Orange,
    #[doc(alias("Blue"))]
    DarkBlue,
    #[doc(alias("Magenta"))]
    Purple,
    #[doc(alias("Cyan"))]
    DarkCyan,
    #[doc(alias("White", "LightGrey"))]
    LightGray,
    #[doc(alias("BrightBlack","Grey"))]
    Gray=90,
    #[doc(alias("BrightRed"))]
    Red,
    #[doc(alias("BrightGreen"))]
    Green,
    #[doc(alias("BrightYellow"))]
    Yellow,
    #[doc(alias("BrightBlue"))]
    Blue,
    #[doc(alias("BrightMangenta"))]
    Magenta,
    #[doc(alias("BrightCyan"))]
    Cyan,
    #[doc(alias("BrightWhite"))]
    White
}
/// Alias definitions
#[allow(non_upper_case_globals)]
impl AnsiColor {
    /// An alias to `Orange`
    pub const DarkYellow:Self  = Self::Orange;
    /// An alias to `LightGray`
    pub const DarkWhite:Self   = Self::LightGray;
    /// An aliaas to `Purple`
    pub const DarkMagenta:Self = Self::Purple;
    /// An alias to `Gray`
    pub const Grey:Self        = Self::Gray;
    /// An alias to `LightGray`
    pub const LightGrey:Self   = Self::Gray;
}

/// Any valid color definition
/// ====================================================================
///
/// This consolidates pallete-based 4-bit colors with 1-byte and 3-byte
/// color systems.
///
/// Note that currently not all terminals do support all color variants,
/// with that in mind you might reconsider using colors other than from
/// 4-bit pallete.
///
/// Color detection API is TBD.
pub enum Color {
    /// ANSI named color, most extensively supported by terminals
    FgPallete(AnsiColor),
    BgPallete(AnsiColor),
    /// Color from standard 255 colors pallete
    FgByte(u8),
    BgByte(u8),
    /// 24-bit true color definition
    FgRGB(u8,u8,u8),
    BgRGB(u8,u8,u8)
}

impl Color {
    pub const fn is_fg(&self)->bool {
        match self {
            Color::FgPallete(_) | Color::FgByte(_) | Color::FgRGB(_,_,_)
                => true,
            Color::BgPallete(_) | Color::BgByte(_) | Color::BgRGB(_,_,_)
                => false
        }
    }
    #[inline] pub const fn is_bg(&self)->bool {
        !self.is_fg()
    }
}