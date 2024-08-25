// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use crate::image::{ImageId, ImageOps};
use crate::os;
use crate::os::{Os};

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

pub struct ImageOpsExecution {
    ops: Box<dyn ImageOps>,
}

impl ImageOpsExecution {
    pub fn new(ops: Box<dyn ImageOps>) -> Self {
        ImageOpsExecution { ops }
    }

    pub fn install(&self) -> Result<ImageId, String> {
        let image = self.ops.image();
        let id = image.id();

        println!("Installing {}...", image);

        self.ops
            .install()
            .map(|_| Self::ok(id.clone(), format!("✅ Install image {}.", id)))
            .map_err(|error| Self::err(id.clone(), format!("❌ Fail to install {}.\n Cause: {}", id, error)))
    }

    pub fn uninstall(&self) -> Result<ImageId, String> {
        let image = self.ops.image();
        let id = image.id();

        println!("Uninstalling {}...", image);

        self.ops
            .uninstall()
            .map(|_| Self::ok(id.clone(), format!("✅ Uninstall image {}.", id)))
            .map_err(|error| Self::err(id.clone(), format!("❌ Fail to uninstall {}.\n Cause: {}", id, error)))
    }

    pub fn reinstall(&self) -> Result<ImageId, String> {
        let image = self.ops.image();
        let id = image.id();

        println!("Reinstalling {}...", image);

        self.ops
            .reinstall()
            .map(|_| Self::ok(id.clone(), format!("✅ Reinstall image {}.", id)))
            .map_err(|error| Self::err(id.clone(), format!("❌ Fail to reinstall {}.\n Cause: {}", id, error)))
    }

    fn ok(id: ImageId, msg: String) -> ImageId {
        println!("{}", msg);

        id
    }

    fn err(id: ImageId, error_msg: String) -> String {
        eprintln!("{}", error_msg);

        id.to_string()
    }
}
