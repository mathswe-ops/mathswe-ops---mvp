// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::iter::Map;
use std::slice::Iter;
use crate::image::ImageId;
use crate::main::system::Operation;
use crate::main::system::Operation::{Install, Reinstall, Uninstall};

pub struct BatchReport {
    ok_num: i32,
    failed: Vec<String>,
}

impl BatchReport {
    pub fn from(
        result: Map<Iter<'_, String>, impl Fn(&String) -> Result<ImageId, String>>
    ) -> Self {
        let empty_report = (0, Vec::new());

        let (ok_num, failed) = result
            .fold(empty_report, Self::success_fail_report);

        BatchReport { ok_num, failed }
    }

    fn success_fail_report(
        acc: (i32, Vec<String>),
        result: Result<ImageId, String>,
    ) -> (i32, Vec<String>) {
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

pub struct BatchOperation {
    pub operation: Operation,
}

impl BatchOperation {
    pub fn execute(
        &self,
        images: &Vec<String>,
        exec: impl Fn(&String) -> Result<ImageId, String>,
    ) -> Result<(), String> {
        let result = images
            .iter()
            .map(exec);

        let report = BatchReport::from(result);

        self.print_batch_report(report)
    }

    pub fn print_batch_report(
        &self,
        BatchReport{ ok_num, failed }: BatchReport,
    ) -> Result<(), String> {
        let report = (ok_num, failed);

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

    fn batch_report_msg(&self, (ok_num, err_ids): (i32, Vec<String>)) -> String {
        match self.operation {
            Install => format!("{} images successfully installed; {} images failed to install.", ok_num, err_ids.len()),
            Uninstall => format!("{} images successfully uninstalled; {} images failed to uninstall.", ok_num, err_ids.len()),
            Reinstall => format!("{} images successfully reinstalled; {} images failed to reinstall.", ok_num, err_ids.len()),
        }
    }

    fn batch_report_success_msg(&self, ok_num: i32) -> String {
        let plural = if ok_num > 1 { "s" } else { "" };

        match self.operation {
            Install => format!("✅ Install {} image{}.", ok_num, plural),
            Uninstall => format!("✅ Uninstall {} image{}.", ok_num, plural),
            Reinstall => format!("✅ Reinstall {} image{}.", ok_num, plural),
        }
    }

    fn batch_report_fail_msg(&self, err_ids: Vec<String>) -> String {
        let plural = if err_ids.len() > 1 { "s" } else { "" };

        match self.operation {
            Install => format!("❌ Fail to install {} image{}: {:?}", err_ids.len(), plural, err_ids),
            Uninstall => format!("❌ Fail to uninstall {} image{}: {:?}", err_ids.len(), plural, err_ids),
            Reinstall => format!("❌ Fail to reinstall {} image{}: {:?}", err_ids.len(), plural, err_ids),
        }
    }
}
