// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use crate::image::{Config, ImageId, ImageOps};

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
            .map(|_| ok(id.clone(), format!("✅ Install image {}.", id)))
            .map_err(|error| err(id.clone(), format!("❌ Fail to install {}.\n Cause: {}", id, error)))
    }

    pub fn uninstall(&self) -> Result<ImageId, String> {
        let image = self.ops.image();
        let id = image.id();

        println!("Uninstalling {}...", image);

        self.ops
            .uninstall()
            .map(|_| ok(id.clone(), format!("✅ Uninstall image {}.", id)))
            .map_err(|error| err(id.clone(), format!("❌ Fail to uninstall {}.\n Cause: {}", id, error)))
    }

    pub fn reinstall(&self) -> Result<ImageId, String> {
        let image = self.ops.image();
        let id = image.id();

        println!("Reinstalling {}...", image);

        self.ops
            .reinstall()
            .map(|_| ok(id.clone(), format!("✅ Reinstall image {}.", id)))
            .map_err(|error| err(id.clone(), format!("❌ Fail to reinstall {}.\n Cause: {}", id, error)))
    }
}

pub struct ConfigExecution {
    ops: Box<dyn Config>,
}

impl ConfigExecution {
    pub fn new(ops: Box<dyn Config>) -> Self {
        ConfigExecution { ops }
    }

    pub fn config(&self) -> Result<ImageId, String> {
        let id = self.ops.image_id();

        println!("Configuring {}...", id);

        self.ops
            .config()
            .map(|_| ok(id.clone(), format!("✅ Config image {}.", id)))
            .map_err(|error| err(
                id.clone(),
                format!("❌ Fail to config {}.\n Cause: {}", id, error),
            ))
    }
}

fn ok(id: ImageId, msg: String) -> ImageId {
    println!("{}", msg);

    id
}

fn err(id: ImageId, error_msg: String) -> String {
    eprintln!("{}", error_msg);

    id.to_string()
}
