// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use crate::main::batch::BatchOperation;
use crate::main::cli::CliCommand::{Install, Reinstall, Uninstall};
use crate::main::exec::{OperationContext, OperationExecution};
use crate::main::system::Operation;
use clap::{Parser, Subcommand};
use std::fmt::{Display, Formatter};
use CliCommand::Config;

#[derive(Subcommand)]
pub enum CliCommand {
    Install {
        #[arg(required = true)]
        images: Vec<String>,

        #[arg(long)]
        config: bool,
    },
    Uninstall {
        #[arg(required = true)]
        images: Vec<String>,
    },
    Reinstall {
        #[arg(required = true)]
        images: Vec<String>,
    },
    Config {
        #[arg(required = true)]
        images: Vec<String>,
    },
}

impl Display for CliCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_operation().fmt(f)
    }
}

impl CliCommand {
    pub fn to_operation(&self) -> Operation {
        match self {
            Install { .. } => Operation::Install,
            Uninstall { .. } => Operation::Uninstall,
            Reinstall { .. } => Operation::Reinstall,
            Config { .. } => Operation::Config,
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        let ctx = OperationContext::load()?;
        let exec = OperationExecution { ctx };
        let batch = BatchOperation { operation: self.to_operation() };

        match self {
            Install { images, config } =>
                batch.execute(images, |id_raw| exec.install(id_raw, config)),

            Uninstall { images } =>
                batch.execute(images, |id_raw| exec.uninstall(id_raw)),

            Reinstall { images } =>
                batch.execute(images, |id_raw| exec.reinstall(id_raw)),

            Config { images } =>
                batch.execute(images, |id_raw| exec.config(id_raw)),
        }
    }
}

#[derive(Parser)]
#[command(name = "system")]
pub struct SystemCli {
    #[command(subcommand)]
    pub operation: CliCommand,
}
