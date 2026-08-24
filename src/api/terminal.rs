// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Terminal information API
//! ====================================================================
//!
//! This api defines method to gather information about current terminal
//! (if any) to adapt its behavior.

use std::io;

mod private {
    /// Marks trait as non-implementable by third-parties.
    pub trait Sealed {}
}

/// An intermediate `io::IsTerminal` trait, to allow for customized
/// implementations while making it compatible with `io:IsTerminal`
/// implementations.
pub trait IsTerminal: private::Sealed {
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

impl private::Sealed for NeverTerminal {}
impl IsTerminal for NeverTerminal {
    fn is_terminal(&self) -> bool { false }
}

/// A global alias to `IsTerminal::is_terminal`
#[inline] pub fn is_terminal<Terminal:IsTerminal>(v:&Terminal)->bool {
    v.is_terminal()
}

pub use crate::sys::terminal::terminal_get_size;
use crate::terminal::private::Sealed;
