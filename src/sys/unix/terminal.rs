// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Platform-agnostic terminal manipulation APIs (POSIX)

use std::{
    num::NonZeroU16 as u16_nz,
    env::var,
    io::stdout,
    os::fd::AsRawFd
};
use libc::{c_int, ioctl, TIOCGWINSZ, winsize};
use crate::terminal::*;

/// Trait to represent raw terminal representation requirements.
pub trait TerminalRaw: AsRawFd {}
impl<T: AsRawFd> TerminalRaw for T {}

impl AsRawFd for NeverTerminal {
    fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
        unimplemented!()
    }
}

/// Get current `(columns,lines)` for terminal buffers.
/// ====================================================================
///
/// This gets current columns/lines in `(X,Y)` format for the chosen
/// file descriptor, or `stdout()` if `None` were given.
/// 
/// Remarks
/// --------------------------------------------------------------------
/// 
/// Key note on this implementation is that env vars `COLUMNS` and
/// `LINES` are preffered over system call. This allows for eventual
/// output format manipulation by users and possible performance
/// benefits.
///
/// `None` value denotes that implementation could not gather info on
/// valid columns/lines or that `fd` was not a terminal. It then should
/// be treated like a buffer had infinite columns by implementations
/// relying on the value.
///
pub fn terminal_get_size<T:AsRawFd+IsTerminal>(fd:Option<&T>)->Option<(u16_nz,u16_nz)> {
    if fd.is_some_and(|t| !t.is_terminal()) { return None; }
    let (mut cols,mut lines)=(0,0);
    // env
    if let Ok(Ok(cols_env)) = var("COLUMNS").map(|cols| cols.parse::<u16>()) {
        cols=cols_env;
    }
    if let Ok(Ok(lines_env)) = var("LINES").map(|cols| cols.parse::<u16>()) {
        lines=lines_env;
    }
    if (cols == 0 || lines == 0) && (fd.is_some()
            || stdout().is_terminal()) {
        // ioctl
        let mut stdout_lock = None;
        let fd = fd.map_or_else(|| {
                stdout_lock = Some(stdout().lock());
                stdout_lock.as_ref().unwrap().as_raw_fd()
            }, |fd| fd.as_raw_fd());
        let mut dim = winsize { ws_col:0, ws_row:0, ws_xpixel:0, ws_ypixel:0 };
        if unsafe { ioctl(fd.as_raw_fd() as c_int, TIOCGWINSZ, &mut dim) } >= 0 {
            if cols  == 0 { cols  = dim.ws_col; }
            if lines == 0 { lines = dim.ws_row; }
        }
    }
    Some((u16_nz::new(cols)?,u16_nz::new(lines)?))
}
