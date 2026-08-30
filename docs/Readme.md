<!--
SPDX-FileCopyrightText: 2026 Dawid Papiewski "SpacingBat3" <spacingbat3@gmail.com>

SPDX-License-Identifier: MPL-2.0
-->

<div align=center>

![](img/logo.png)

Kontur
========================================================================

A terminal manipulation library for CLI applications.

</div>

Description
------------------------------------------------------------------------

Kontur aims to fully implement a terminal manipulation for apps that
don't need entire stack of ANSI sequences for cursor positioning,
alternative screen buffers or component drawing. It also implements
its abstractions to build upon and extend `std` related traits like
`IsTerminal` than to do its own thing from scratch.

### Motivation

There are many great and high-quality TUI frameworks for Rust, with
[`ratatui`] being one of notable examples. However, not everything you
might wish to implement might be a TUI and sometimes CLI allows for
more scriptable and faster control interface to both implement and use.

While there are also many terminal manipulations libraries that could
also be integrated into CLI software directly, Kontur is supposed to
provide you a minimalistic API just needed to get a job done, than to
overwhelm you with functionalities you might not neccesarily need when
designing your software. 

### Name Origin

*Kontur* is a Polish word, that means *outline*. It tries to represent
the word *line* in *command-line interface* phrase, to signify that
manipulation focuses not around whole terminal window, as seen in many
TUI frameworks, but rather just the current line of it.

`[feature]` flags
------------------------------------------------------------------------

 - `ascii` – ASCII-focused variants of functions, for programs that
   do not want to handle full unicode support.

 - `unicode` – an unicode-aware text manipulations support, via
   `unicode-width` crate.

[`ratatui`]: https://ratatui.rs/

License
------------------------------------------------------------------------

© 2026 Dawid Papiewski "[SpacingBat3]" <spacingbat3@gmail.com>

This project is subject to the terms of the
[Mozilla Public License, v. 2.0][MPL-2.0]. If a copy of the MPL was not
distributed with this file, You can obtain one at
<https://mozilla.org/MPL/2.0/>.

[SpacingBat3]: https://github.com/SpacingBat3 "GitHub Profile"
[MPL-2.0]:     https://mozilla.org/MPL/2.0/   "Identified by SPDX: MPL-2.0"
