// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use clap::{Parser, Subcommand};
use std::fmt::{Display, Formatter};

use crate::image::repository::Repository;
use crate::image::{ImageId, ImageOps};
use crate::main::cli::CliCommand::{Install, Reinstall, Uninstall};
use crate::main::exec::{OperationContext};
use crate::main::image_exec::ImageOpsExecution;
use crate::main::system::Operation;
use crate::os::Os;

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
        let os = ctx.os;

        // 0: Number of Ok results, 1: List of IDs that failed
        let empty_report = (0, Vec::new());

        type Exec = fn(ImageOpsExecution) -> Result<ImageId, String>;

        let process_image_with
            = |exec: Exec| move |id_raw: &String| Self::load_image_ops(id_raw, &os)
            .map(ImageOpsExecution::new)
            .and_then(exec);

        let batch_image_op = |images: &Vec<String>, exec: Exec| {
            let report = images
                .iter()
                .map(process_image_with(exec))
                .fold(empty_report, Self::success_fail_report);

            self.image_batch_report(report)
        };

        match self {
            Install { images } => batch_image_op(images, |exec| exec.install()),
            Uninstall { images } => batch_image_op(images, |exec| exec.uninstall()),
            Reinstall { images } => batch_image_op(images, |exec| exec.reinstall()),
        }
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

    fn image_batch_report(&self, report: (i32, Vec<String>)) -> Result<(), String> {
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

    fn load_image_ops(id_raw: &str, os: &Os) -> Result<Box<dyn ImageOps>, String> {
        Self::load_image(id_raw, os.clone())
            .map_err(|error| {
                println!("{}", format!("❌ Fail to load image {}.\nCause: {}", id_raw, error));
                id_raw.to_string()
            })
    }

    fn load_image(id_raw: &str, os: Os) -> Result<Box<dyn ImageOps>, String> {
        Repository::image_loader_from(id_raw)
            .and_then(|loader| loader
                .load_image(os.clone())
                .map_err(|error| error.to_string())
            )
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
