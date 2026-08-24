// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Style contructor macro
/// ====================================================================
///
/// A syntatic sugar macro to operate on complex enum system that was
/// currently adopted for Kontur.
///
/// Usage
/// --------------------------------------------------------------------
///
/// ```
/// // Like `Style::Attribute(Attribute::Bold)`
/// style!(attribute(Bold))
/// // For colors:
/// style!(color(Red))
/// // Or if you want to be explicit:
/// style!(fg::color(Red))
/// // If you want to set blue background color instead:
/// style!(bg::color(Blue))
/// // Also red, but via RGB, not ANSI pallete
/// style!(rgb(255,0,0))
/// style!(hex(0xff0000))
/// // Also `bg`:
/// style!(bg::rgb(0,0,255))
/// style!(bg::hex(0x0000ff))
/// ```
///
#[macro_export]
macro_rules! style {
    //
    // ATTRIBUTE SUGAR
    //
    (attribute($a:ident)) => {
        $crate::style::Style::Attribute($crate::style::Attribute::$a)
    };
    //
    // COLOR SUGAR
    //
    (color($c:ident)) => {
        $crate::style::Style::Color($crate::style::Color::FgPallete(
            $crate::style::AnsiColor::$c
        ))
    };
    (rgb($r:expr,$g:expr,$b:expr)) => {
        $crate::style::Style::Color($crate::style::Color::FgRGB(
            ($r) as u8,($g) as u8,($b) as u8
        ))
    };
    (hex($h:expr)) => {
        $crate::style::Style::Color($crate::style::Color::FgRGB(
            (($h)>>16) as u8,((($h)>>8)&0xff) as u8,(($h)&0xff) as u8
        ))
    };
    //
    // FG::COLOR SUGAR
    //
    (fg::color($c:ident)) => {
        $crate::style::Style::Color($crate::style::Color::FgPallete(
            $crate::style::AnsiColor::$c
        ))
    };
    (fg::rgb($r:expr,$g:expr,$b:expr)) => {
        $crate::style::Style::Color($crate::style::Color::FgRGB(
            ($r) as u8,($g) as u8,($b) as u8
        ))
    };
    (fg::hex($h:expr)) => {
        $crate::style::Style::Color($crate::style::Color::FgRGB(
            (($h)>>16) as u8,((($h)>>8)&0xff) as u8,(($h)&0xff) as u8
        ))
    };
    //
    // BG::COLOR SUGAR
    //
    (bg::color($c:ident)) => {
        $crate::style::Style::Color($crate::style::Color::BgPallete(
            $crate::style::AnsiColor::$c
        ))
    };
    (bg::rgb($r:expr,$g:expr,$b:expr)) => {
        $crate::style::Style::Color($crate::style::Color::BgRGB(
            ($r) as u8,($g) as u8,($b) as u8
        ))
    };
    (bg::hex($h:expr)) => {
        $crate::style::Style::Color($crate::style::Color::BgRGB(
            (($h)>>16) as u8,((($h)>>8)&0xff) as u8,(($h)&0xff) as u8
        ))
    };
}

/// Text styling macro
/// ====================================================================
///
/// A macro that adopts operations on enums into syntatic sugar syntax
/// inspired by Node.js `util.styleText` API. It allows for array-like
/// or single-element syntax for chaining style application, with last
/// replacing the previous ones in case of the conflict (same behavior
/// as in Node). Additionally, it covers `reset` operation for
/// attribute groups meant to be used only this way (mainly `Reset`
/// attribute, which might not be that much useful for wrapping text)
/// or as additional measure to ensure no externally set styles won't
/// leak past given position in text.
///
#[macro_export]
macro_rules! style_text {
    // FIXME: opportunity to introduce apply_all to remove
    //        function chaining and introduce non-sugar syntax
    ([$style:expr,$($rest:expr),+],$text:expr) => {
        $style.apply(style_text!([$($rest),+],$text))
    };
    ([$style:expr],$text:expr) => {
        $style.apply($text)
    };
    (reset $style:expr,$text:expr) => {
        $style.barrier($text, $text.len())
    };
    (reset($pos:literal) $style:expr,$text:expr) => {
        $style.barrier($text, $pos)
    };
    ($style:expr,$text:expr) => {
        $style.apply($text)
    };
}

pub use {style,style_text};
