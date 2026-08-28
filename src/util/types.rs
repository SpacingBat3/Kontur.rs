// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later OR GPL-3.0-or-later

use std::{hint::cold_path, sync::atomic::{AtomicU8, Ordering}};

/// A 3-state bool representation.
#[repr(u8)]
#[derive(Default)]
enum MaybeBoolValue {
    False,
    True,
    #[default]
    Uninitialized
}

use MaybeBoolValue::*;

impl From<bool> for MaybeBoolValue {
    #[inline] fn from(value: bool) -> Self {
        if value { MaybeBoolValue::True }
        else     { MaybeBoolValue::False }
    }
}

impl From<MaybeBoolValue> for AtomicU8 {
    fn from(value: MaybeBoolValue) -> Self {
        Self::new(value as u8)
    }
}

/// This is atomic 3-state representation type, which is constructed
/// similarly to OnceLock, but might represent AtomicBool more optimally
/// in memory layout and be initialized to 2-state representation more
/// times than once, although still being optimized for single
/// initialization cases and expecting 2-state bool value when taking
/// code paths.
pub(crate) struct MaybeBool {
    #[doc(hidden)]
    _repr: AtomicU8
}

impl Default for MaybeBool {
    #[inline] fn default() -> Self {
        Self { _repr: AtomicU8::from(Uninitialized) }
    }
}

impl MaybeBool {
    #[inline] pub(crate) const fn new()->Self {
        Self { _repr: AtomicU8::new(Uninitialized as u8) }
    }
    #[inline] pub(crate) fn set(&self,value:bool) {
        self._repr.store(
            MaybeBoolValue::from(value) as u8,
            Ordering::Release
        )
    }
    #[inline] pub(crate) fn reset(&self) {
        self._repr.store(
            MaybeBoolValue::Uninitialized as u8,
            Ordering::Release
        )
    }
    pub(crate) fn get_or_init(&self,initializer:fn()->bool)->bool {
        let mut state = self._repr.load(Ordering::Acquire);
        if state == Uninitialized as u8 {
            cold_path();
            state = MaybeBoolValue::from(initializer()) as u8;
            self._repr.store(state, Ordering::Release);
        }
        state == (True as u8)
    }
}
