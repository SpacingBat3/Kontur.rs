// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

#![doc = include_str!("../docs/Readme.md")]
#![doc(html_playground_url = "https://play.rust-lang.org/")]

// Prelude recommendations
pub mod prelude;

/// Library public API
mod api {
    pub mod terminal;
    pub mod fmt;
    pub mod style;
    pub mod hook;
}
pub use api::*;

/// Macros
mod macros {
    pub(crate) mod style;
}

/// Utilities unrelated to API
mod util {
   pub(crate) mod types; 
}

/// Platform-dependant implementations
#[cfg_attr(unix,    path = "sys/unix.rs"   )]
#[cfg_attr(windows, path = "sys/windows.rs")]
pub(crate) mod sys;
