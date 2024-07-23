// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::io;

use clap::{Parser, Subcommand};

use crate::package::{Os, Package, UBUNTU_X64};
use crate::image::{Install, Uninstall};
use crate::image::images::{ImageId, load_image};

mod tmp;
mod download;
mod resources;
mod cmd;
mod image;
mod package;

#[derive(Parser)]
#[command(name = "system")]
struct System {
    #[command(subcommand)]
    operation: Operation,
}

#[derive(Subcommand)]
enum Operation {
    Install {
        #[arg(required = true)]
        images: Vec<ImageId>,
    },
    Uninstall {
        #[arg(required = true)]
        images: Vec<ImageId>,
    },
}

pub fn detect_os() -> io::Result<Option<Os>> {
    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        let os_release = std::fs::read_to_string("/etc/os-release")?;

        if os_release.contains("Ubuntu") {
            Ok(Some(UBUNTU_X64))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn execute_operation(operation: Operation) -> Result<(), String> {
    let os = detect_os()
        .map_err(|io_error| io_error.to_string())?
        .ok_or_else(|| "OS unsupported".to_string())?;

    let load = |id: ImageId| load_image(id.clone(), os.clone())
        .ok_or_else(|| format!("Image {} not supported", id));

    match operation {
        Operation::Install { images: packages } => {
            for id in packages {
                let image = load(id)?;

                println!("Installing {}...", image);
                image.install()?
            }
            Ok(())
        }
        Operation::Uninstall { images: packages } => {
            for id in packages {
                let image = load(id)?;

                println!("Uninstalling {}...", image);
                image.uninstall()?
            }

            Ok(())
        }
    }
}

fn main() {
    let cli = System::parse();
    let exec = execute_operation(cli.operation);

    match exec {
        Ok(_) => println!("Execution successful"),
        Err(err) => eprintln!("{}", format!("Fail to execute: {}", err))
    }
}
