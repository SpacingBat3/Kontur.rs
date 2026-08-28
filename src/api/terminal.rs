// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Terminal information API
//! ====================================================================
//!
//! This api defines method to gather information about current terminal
//! (if any) to adapt its behavior.

use std::io;
#[cfg(any(unix,windows))]
use std::num::NonZeroU16;

mod private {
    /// Marks trait as non-implementable by third-parties.
    pub trait Sealed {}
}
use private::Sealed;

/// `io::isTerminal` extensions
/// ====================================================================
///
/// This is an intermediate type that is functionally similar / same
/// to `io::isTerminal`, but allows this crate to extend it to more
/// types. For external crates it still remains sealed, like
/// `io::IsTerminal`.
///
pub trait IsTerminal: Sealed {
    fn is_terminal(&self)->bool;
}

impl<T:io::IsTerminal> Sealed for T {}
impl<T:io::IsTerminal> IsTerminal for T {
    fn is_terminal(&self)->bool {
        io::IsTerminal::is_terminal(self)
    }
}

/// A fake `IsTerminal` type that panics on use as fd/handle.
pub struct NeverTerminal;

impl Sealed for NeverTerminal {}
impl IsTerminal for NeverTerminal {
    fn is_terminal(&self) -> bool { false }
}

/// A global alias to `IsTerminal::is_terminal`
#[inline] pub fn is_terminal<Terminal:IsTerminal>(v:&Terminal)->bool {
    v.is_terminal()
}

#[cfg(any(unix,windows))]
pub use crate::sys::terminal::terminal_get_size;

/// A platform-agnostic `io::IsTerminal` extensions
/// ====================================================================
/// 
/// This extends `io::IsTerminal` that also satisfy platform-specific
/// API requirements to offer extended features.
/// 
/// Note: this is implemented for `windows` or `unix` with
/// otherwise API-compatible shim that does nothing.
/// 
#[cfg(any(unix,windows))]
pub trait PlatformTerminal: Sized + crate::sys::terminal::TerminalRaw
        + IsTerminal {
    /// Indicates if current platform is supported
    const SUPPORTED_PLATFORM:bool = true;
    /// Get `(columns,rows)` if this is a terminal, or `None` otherwise.
    /// ================================================================
    ///
    /// This is the same as calling `terminal::terminal_get_size`
    ///
    /// Remarks
    /// ----------------------------------------------------------------
    /// 
    /// Additionally on `unix`, this will check beforehand if `COLUMNS`
    /// and `LINES` is being provided in env before issuing system call.
    /// This both might improve performance and allow users to override
    /// the behavior if needed.
    /// 
    #[inline] fn terminal_get_size(&self)->Option<(NonZeroU16,NonZeroU16)> {
        terminal_get_size(Some(self))
    }
}

#[cfg(any(unix,windows))]
impl <T:Sized+crate::sys::terminal::TerminalRaw+IsTerminal>
PlatformTerminal for T {}

/// A platform-agnostic `io::IsTerminal` extensions
/// ====================================================================
/// 
/// This is a shim for unsupported platforms. It uses deprecation notice
/// to warn about itself about all shimmed use cases
/// 
#[cfg(not(any(unix,windows)))]
pub trait IsPlatformTerminal: Sized + IsTerminal {
    /// Indicates if current platform is supported
    const SUPPORTED_PLATFORM:bool = false;
    /// Get `(columns,rows)` if this is a terminal, or `None` otherwise.
    /// ================================================================
    ///
    /// For unsupported (this!) platform, this always returns `None`.
    /// 
    #[deprecated = concat!(
        "This API is not supported on your platform, it will use shim ",
        "to keep a dependant compiled code compatible. You might request ",
        "the platform support at https://github.com/SpacingBat3/Kontur.rs."
    )]
    #[inline] fn terminal_get_size(&self)->Option<(NonZeroU16,NonZeroU16)> {
        None
    }
}
