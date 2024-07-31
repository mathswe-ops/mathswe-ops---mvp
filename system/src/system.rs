// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use clap::{Parser, Subcommand};
use crate::image::ImageOps;
use crate::image::repository::Repository;
use crate::os::detect_os;
use crate::system::Operation::{Install, Reinstall, Uninstall};

#[derive(Parser)]
#[command(name = "system")]
pub struct System {
    #[command(subcommand)]
    pub(crate) operation: Operation,
}

#[derive(Subcommand)]
pub enum Operation {
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

impl Operation {
    pub fn execute(&self) -> Result<(), String> {
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

        match self {
            Install { images } => {
                for id_raw in images {
                    let ops = load_image(id_raw.to_string())?.unwrap();

                    println!("Installing {}...", ops.image());
                    ops.install()?
                }
                Ok(())
            }
            Uninstall { images } => {
                for id_raw in images {
                    let ops = load_image(id_raw.to_string())?.unwrap();

                    println!("Uninstalling {}...", ops.image().id());
                    ops.uninstall()?
                }

                Ok(())
            }
            Reinstall { images } => {
                for id_raw in images {
                    let ops = load_image(id_raw.to_string())?.unwrap();

                    println!("Reinstalling {}...", ops.image());
                    ops.reinstall()?
                }

                Ok(())
            }
        }
    }
}
