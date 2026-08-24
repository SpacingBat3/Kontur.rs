// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

#![doc = include_str!("../docs/Readme.md")]
#![doc(html_playground_url = "https://play.rust-lang.org/")]

// Prelude recommendations
pub mod prelude;

// Library public API
mod api {
    pub mod terminal;
    pub mod fmt;
    pub mod style;
    pub mod hook;
}
pub use api::*;

// Macros
pub mod macros {
    pub mod style;
}

#[cfg_attr(unix,    path = "sys/unix.rs"   )]
#[cfg_attr(windows, path = "sys/windows.rs")]
// Platform-dependant implementations
pub(crate) mod sys;
