// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::io;

use clap::{Parser, Subcommand};

use crate::image::{ImageOps};
use crate::image::repository::Repository;
use crate::package::{Os, UBUNTU_X64};

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
        images: Vec<String>,
    },
    Uninstall {
        #[arg(required = true)]
        images: Vec<String>,
    },
    Reinstall {
        #[arg(required = true)]
        images: Vec<String>,
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
    type LoadResult = Result<Option<Box<dyn ImageOps>>, String>;

    let os = detect_os()
        .map_err(|io_error| io_error.to_string())?
        .ok_or_else(|| "OS unsupported".to_string())?;

    let load_image = |id_raw: String| -> LoadResult {
        Repository::image_loader_from(&id_raw)
            .and_then(|loader| loader
                .load_image(os.clone())
                .map_err(|error| error.to_string())
            )
    };

    match operation {
        Operation::Install { images: packages } => {
            for id_raw in packages {
                let ops = load_image(id_raw)?.unwrap();

                println!("Installing {}...", ops.image());
                ops.install()?
            }
            Ok(())
        }
        Operation::Uninstall { images: packages } => {
            for id_raw in packages {
                let ops = load_image(id_raw)?.unwrap();

                println!("Uninstalling {}...", ops.image());
                ops.uninstall()?
            }

            Ok(())
        }
        Operation::Reinstall { images: packages } => {
            for id_raw in packages {
                let ops = load_image(id_raw)?.unwrap();

                println!("Reinstalling {}...", ops.image());
                ops.reinstall()?
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
