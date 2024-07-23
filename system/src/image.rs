// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

pub(crate) mod images;

pub trait Install {
    fn install(&self) -> Result<(), String>;
}

pub trait Uninstall {
    fn uninstall(&self) -> Result<(), String>;
}
