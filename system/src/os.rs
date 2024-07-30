// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use LinuxType::Ubuntu;
use OsArch::X64;
use crate::os::Os::Linux;

#[derive(Clone, Debug)]
pub enum OsArch {
    X64
}

#[derive(Clone, Debug)]
pub enum LinuxType {
    Ubuntu
}

#[derive(Clone, Debug)]
pub enum Os {
    Linux(OsArch, LinuxType)
}

pub const UBUNTU_X64: Os = Linux(X64, Ubuntu);
