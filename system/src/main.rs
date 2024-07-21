// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use clap::{Parser, Subcommand};

use Operation::Uninstall;

use crate::Operation::Install;
use crate::package::packages::Package;

mod tmp;
mod download;
mod resources;
mod cmd;
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
        packages: Vec<String>,
    },
    Uninstall {
        #[arg(required = true)]
        packages: Vec<String>,
    },
}

fn execute_operation(operation: Operation) -> Result<(), String> {
    let get_package = |name: String| Package::from(name);

    match operation {
        Install { packages } => {
            for package_name in packages {
                let package = get_package(package_name)?;

                println!("Installing {}...", package);
            }

            Ok(())
        }
        Uninstall { packages } => {
            for package_name in packages {
                let package = get_package(package_name)?;

                println!("Uninstalling {}...", package);
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
