// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use clap::{Parser, Subcommand};
use Operation::Uninstall;
use crate::Operation::Install;

mod tmp;
mod download;
mod resources;
mod cmd;

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
        packages: Vec<String>,
    },
    Uninstall {
        #[arg(required = true)]
        packages: Vec<String>,
    },
}

fn main() {
    let cli = System::parse();

    match cli.operation {
        Install { packages } => {
            for package in packages {
                println!("Installing {}", package);
            }
        }
        Uninstall { .. } => {
            println!("Uninstall operation unsupported")
        }
    }
}
