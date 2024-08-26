// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use crate::image::repository::Repository;
use crate::image::{ImageId, ImageOps};
use crate::main::image_exec::ImageOpsExecution;
use crate::os;
use crate::os::Os;

#[derive(Clone)]
pub struct OperationContext {
    os: Os,
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

    fn load_image_ops(
        &self,
        id_raw: &str,
    ) -> Result<Box<dyn ImageOps>, String> {
        self.load_image(id_raw)
            .map_err(|error| {
                println!("{}", format!("❌ Fail to load image {}.\nCause: {}", id_raw, error));
                id_raw.to_string()
            })
    }

    fn load_image(
        &self,
        id_raw: &str,
    ) -> Result<Box<dyn ImageOps>, String> {
        Repository::image_loader_from(id_raw)
            .and_then(|loader| loader
                .load_image(self.os.clone())
                .map_err(|error| error.to_string())
            )
    }
}

#[derive(Clone)]
pub struct OperationExecution {
    pub ctx: OperationContext,
}

impl OperationExecution {
    pub fn install(
        &self,
        id_raw: &String,
    ) -> Result<ImageId, String> {
        let ops = self.ctx.load_image_ops(id_raw)?;
        let exec = ImageOpsExecution::new(ops);

        exec.install()
    }

    pub fn uninstall(
        &self,
        id_raw: &String,
    ) -> Result<ImageId, String> {
        let ops = self.ctx.load_image_ops(id_raw)?;
        let exec = ImageOpsExecution::new(ops);

        exec.uninstall()
    }

    pub fn reinstall(
        &self,
        id_raw: &String,
    ) -> Result<ImageId, String> {
        let ops = self.ctx.load_image_ops(id_raw)?;
        let exec = ImageOpsExecution::new(ops);

        exec.reinstall()
    }
}
