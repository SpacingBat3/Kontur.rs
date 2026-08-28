// SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>
//
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Hook API
//! ====================================================================
//!
//! Defines "middleware" logic that allows for inteligent adaptation of
//! Kontur to given environments.
//!

use std::{
    env::Args,
    iter::Filter
};
use crate::style::ALLOW_STYLES;

fn filter_fn<'a>(arg:&'a String)->bool {
    match arg.split_once("=").unwrap_or((arg,"always")) {
        ("--color", value) => {match value {
            "always"|"yes"|"true"   => ALLOW_STYLES.set(true),
            "never" |"no" |"false"  => ALLOW_STYLES.set(false),
            "auto"  |"tty"|"if-tty" => ALLOW_STYLES.reset(),
            _ => return true /* Unsupported arg */
        }; false}
        _ => true
    }
}

mod private {
    pub trait Sealed {}
}
use private::Sealed;

impl Sealed for Args {}
impl<I,P> Sealed for Filter<I,P>
where Filter<I, P>:IntoIterator<Item = String>, I:KonturHook+Sealed {}

/// Trait to hook Kontur into `std::env::Args`
/// ====================================================================
/// 
/// Hooks work similarly to `filter()`, with an exception that it is
/// allowed to use `KonturHook` anywhere in filter chain, not just on
/// `Args` directly. This, in theory, allows for others to hook into
/// `args` as long as they do so by filter chain.
/// 
/// Remarks
/// --------------------------------------------------------------------
/// 
/// The `KonturHook` is sealed trait, you might not be able to implement
/// it for your own types.
/// 
pub trait KonturHook:IntoIterator<Item = String>+Sized+private::Sealed {
    /// Adds hook to a filter chain.
    fn use_kontur_hook(self)->Filter<Self::IntoIter, impl FnMut(&Self::Item) -> bool>;
}

impl KonturHook for Args {
    fn use_kontur_hook(self)->Filter<Args, impl FnMut(&Self::Item) -> bool> {
        self.filter(filter_fn)
    }
}

impl<I,P> KonturHook for Filter<I,P>
where Filter<I, P>:IntoIterator<Item = String>, I:KonturHook+private::Sealed {
    fn use_kontur_hook(self)->Filter<<Filter<I, P> as IntoIterator>::IntoIter, impl FnMut(&Self::Item) -> bool> {
        self.into_iter().filter(filter_fn)
    }
}
