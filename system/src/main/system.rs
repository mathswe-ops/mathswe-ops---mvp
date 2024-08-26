// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use Operation::Config;
use crate::main::system::Operation::{Install, Reinstall, Uninstall};

#[derive(Clone)]
pub enum Operation {
    Install,
    Uninstall,
    Reinstall,
    Config,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Install => "install",
            Uninstall => "uninstall",
            Reinstall => "reinstall",
            Config => "config",
        };

        write!(f, "{}", msg)
    }
}
