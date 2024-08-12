// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use LinuxType::Ubuntu;
use OsArch::X64;
use PkgType::Deb;
use crate::cmd::exec_cmd;
use crate::os::Os::Linux;

#[derive(PartialEq, Clone, Debug)]
pub enum OsArch {
    X64
}

#[derive(PartialEq, Clone, Debug)]
pub enum LinuxType {
    Ubuntu
}

#[derive(PartialEq, Clone, Debug)]
pub enum Os {
    Linux(OsArch, LinuxType)
}

pub const UBUNTU_X64: Os = Linux(X64, Ubuntu);

pub enum PkgType {
    Deb
}

pub struct OsPkg {
    pub pkg_type: PkgType,
    pub name: String,
}

impl OsPkg {
    pub fn install(&self, installer_path: &PathBuf) -> Result<(), String> {
        match self.pkg_type {
            Deb => Self::install_deb(installer_path)
        }
    }

    pub fn uninstall(&self) -> Result<(), String> {
        match self.pkg_type {
            Deb => Self::uninstall_deb(&self.name)
        }
    }

    fn install_deb(installer: &PathBuf) -> Result<(), String> {
        let output = exec_cmd(
            "sudo",
            &["apt-get", "--yes", "install", installer.to_str().unwrap()],
        ).map_err(|error| error.to_string())?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        println!("{}", stdout);

        Ok(())
    }

    fn uninstall_deb(name: &str) -> Result<(), String> {
        println!("{}", format!("Removing package {}...", name));

        let output = exec_cmd(
            "sudo",
            &["apt-get", "--yes", "remove", name],
        ).map_err(|error| error.to_string())?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        println!("{}", stdout);

        println!("Cleaning up no longer required packages...");

        let output = exec_cmd(
            "sudo",
            &["apt-get", "--yes", "autoremove"],
        ).map_err(|error| error.to_string())?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        println!("{}", stdout);

        Ok(())
    }
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

/// Notice: It may return a list of truncated process names, so check for
/// prefixes when trying to find a process name. For example, it may return
/// "jetbrains-toolb" instead of "jetbrains-toolbox."
pub fn get_running_processes(os: Os) -> Result<Vec<String>, String> {
    match os {
        Linux(X64, Ubuntu) => get_running_processes_ubuntu()
    }
}

fn get_running_processes_ubuntu() -> Result<Vec<String>, String> {
    let output = exec_cmd("ps", &["-e", "-o", "comm="])
        .map_err(|error| error.to_string())?;

    let reader = BufReader::new(&output.stdout[..]);
    let processes = reader
        .lines()
        .filter_map(|line| line.ok())  // Remove any errors
        .collect::<Vec<String>>();

    Ok(processes)
}

pub fn kill_process(os: Os, process_name: &str) -> Result<(), String> {
    match os {
        Linux(X64, Ubuntu) => kill_process_ubuntu(process_name)
    }
}

fn kill_process_ubuntu(process_name: &str) -> Result<(), String> {
    exec_cmd("killall", &[process_name])
        .map_err(|error| error.to_string())?;

    Ok(())
}
