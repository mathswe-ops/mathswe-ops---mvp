// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use crate::image::{ImageId};
use crate::main::cli::CliCommand::{Install, Reinstall, Uninstall};
use crate::main::exec::{OperationContext, OperationExecution};
use crate::main::system::Operation;
use clap::{Parser, Subcommand};
use std::fmt::{Display, Formatter};

#[derive(Subcommand)]
pub enum CliCommand {
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
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        let ctx = OperationContext::load()?;
        let exec = OperationExecution { ctx };

        match self {
            Install { images } =>
                self.execute_batch(images, |id_raw| exec.install(id_raw)),

            Uninstall { images } =>
                self.execute_batch(images, |id_raw| exec.uninstall(id_raw)),

            Reinstall { images } =>
                self.execute_batch(images, |id_raw| exec.reinstall(id_raw)),
        }
    }

    fn execute_batch(
        &self,
        images: &Vec<String>,
        exec: impl Fn(&String) -> Result<ImageId, String>,
    ) -> Result<(), String> {
        // 0: Number of Ok results, 1: List of IDs that failed
        let empty_report = (0, Vec::new());

        let report = images
            .iter()
            .map(exec)
            .fold(empty_report, Self::success_fail_report);

        self.print_batch_report(report)
    }

    fn batch_report_msg(&self, (ok_num, err_ids): (i32, Vec<String>)) -> String {
        match self.to_operation() {
            Operation::Install => format!("{} images successfully installed; {} images failed to install.", ok_num, err_ids.len()),
            Operation::Uninstall => format!("{} images successfully uninstalled; {} images failed to uninstall.", ok_num, err_ids.len()),
            Operation::Reinstall => format!("{} images successfully reinstalled; {} images failed to reinstall.", ok_num, err_ids.len()),
        }
    }

    fn batch_report_success_msg(&self, ok_num: i32) -> String {
        let plural = if ok_num > 1 { "s" } else { "" };

        match self.to_operation() {
            Operation::Install => format!("✅ Install {} image{}.", ok_num, plural),
            Operation::Uninstall => format!("✅ Uninstall {} image{}.", ok_num, plural),
            Operation::Reinstall => format!("✅ Reinstall {} image{}.", ok_num, plural),
        }
    }

    fn batch_report_fail_msg(&self, err_ids: Vec<String>) -> String {
        let plural = if err_ids.len() > 1 { "s" } else { "" };

        match self.to_operation() {
            Operation::Install => format!("❌ Fail to install {} image{}: {:?}", err_ids.len(), plural, err_ids),
            Operation::Uninstall => format!("❌ Fail to uninstall {} image{}: {:?}", err_ids.len(), plural, err_ids),
            Operation::Reinstall => format!("❌ Fail to reinstall {} image{}: {:?}", err_ids.len(), plural, err_ids),
        }
    }

    fn print_batch_report(&self, report: (i32, Vec<String>)) -> Result<(), String> {
        match report.clone() {
            (ok_num, err_ids) if err_ids.is_empty() => {
                println!("{}", self.batch_report_success_msg(ok_num));
                Ok(())
            }
            (_, err_ids) => {
                println!("{}", self.batch_report_fail_msg(err_ids));
                Err(self.batch_report_msg(report))
            }
        }
    }

    fn success_fail_report(acc: (i32, Vec<String>), result: Result<ImageId, String>) -> (i32, Vec<String>) {
        let add_element = |mut list: Vec<String>, element: String| -> Vec<String> {
            list.push(element);
            list
        };

        match result {
            Ok(_) => (acc.0 + 1, acc.1),
            Err(id_raw) => (acc.0, add_element(acc.1, id_raw)),
        }
    }
}

#[derive(Parser)]
#[command(name = "system")]
pub struct SystemCli {
    #[command(subcommand)]
    pub operation: CliCommand,
}
