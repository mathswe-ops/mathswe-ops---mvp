// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp


use clap::{Parser};

use crate::system::{System};

mod tmp;
mod download;
mod resources;
mod cmd;
mod image;
mod package;
mod os;
mod system;
mod exec;

fn main() {
    let cli = System::parse();
    let exec = cli.operation.execute();

    match exec {
        Ok(_) => println!("Execution successful"),
        Err(err) => eprintln!("{}", format!("Fail to execute: {}", err))
    }
}
