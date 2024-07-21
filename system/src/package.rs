// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

pub(crate) mod packages;

pub trait Install<E> {
    fn install(&self) -> Result<(), E>;
}

pub trait Uninstall<E> {
    fn uninstall(&self) -> Result<(), E>;
}
