// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use crate::os;
use crate::os::Os;

#[derive(Clone)]
pub struct OperationContext {
    pub(crate) os: Os,
}

impl OperationContext {
    pub fn new(os: Os) -> Self {
        OperationContext { os }
    }

    pub fn load() -> Result<Self, String> {
        os::detect_os()
            .map_err(|io_error| io_error.to_string())?
            .ok_or_else(|| "OS unsupported".to_string())
            .map(OperationContext::new)
    }
}
