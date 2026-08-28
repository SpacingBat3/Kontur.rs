// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
// SPDX-FileCopyrightText: 2026, 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later OR GPL-3.0-or-later

//! Platform-agnostic terminal manipulation APIs (Windows)

use std::{
    num::NonZeroU16 as u16_nz,
    os::windows::io::AsRawHandle,
    io::stdout,
};
use windows::Win32::{
    Foundation::HANDLE as Handle,
    System::Console::{
        CONSOLE_SCREEN_BUFFER_INFO as ConInfo,
        GetConsoleScreenBufferInfo as con_info
    }
};

use crate::terminal::*;

/// Trait to represent raw terminal representation requirements.
pub trait TerminalRaw: AsRawHandle {}
impl<T: AsRawHandle> TerminalRaw for T {}

impl AsRawHandle for NeverTerminal {
    fn as_raw_handle(&self) -> std::os::windows::prelude::RawHandle {
        unimplemented!()
    }
}

/// Get current `(columns,lines)` for terminal buffers.
/// ====================================================================
///
/// This gets current columns/lines in `(X,Y)` format for the chosen
/// console output handle, or `stdout()` if `None` were given.
///
/// Remarks
/// --------------------------------------------------------------------
///
/// `None` value denotes that implementation could not gather info on
/// valid columns/lines or that `con` was not a terminal. It then should
/// be treated like a buffer had infinite columns/lines by
/// implementations relying on the value.
///
pub fn terminal_get_size<T:AsRawHandle+IsTerminal>(con:Option<&T>)->Option<(u16_nz,u16_nz)> {
    if con.is_some_and(is_terminal) || (con.is_none() && stdout().is_terminal()) {
        // Win32 API
        let mut stdout_lock = None;
        let con = con.map_or_else(|| {
                stdout_lock = Some(stdout().lock());
                stdout_lock.as_ref().unwrap().as_raw_handle()
            }, |fd| fd.as_raw_handle());
        let mut info = ConInfo::default();
        if unsafe { con_info(Handle(con), &mut info) }.is_ok() {
            return Some((
                u16_nz::new(info.dwSize.X as u16)?,
                u16_nz::new(info.dwSize.Y as u16)?
            ))
        }
    }
    // fallback
    None
}
