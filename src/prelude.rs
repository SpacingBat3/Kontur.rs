// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later OR GPL-3.0-or-later

//! The Kontur prelude
//! ====================================================================
//! 
//! This defines a set of traits and macros you might need to import to
//! utilize all `kontur` features.
//! 

// Traits
pub use crate::fmt::TtyFormat;
pub use crate::terminal::{IsTerminal,PlatformTerminal};
pub use crate::hook::KonturHook;
// Macros
pub use crate::macros::style::*;
