// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later OR GPL-3.0-or-later

//! OS-agnostic API
//! ====================================================================
//!
//! These are APIs that cannot avoid platform-specific implementation,
//! as no API that guarantees common behavior exists.
//!
//! **Note:** You are currently browsing the `windows` API. Additionally,
//! `unix` is also a supported target.


#[path="windows/terminal.rs"]
pub(crate) mod terminal;
