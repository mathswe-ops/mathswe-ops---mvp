// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use crate::cmd::{exec_cmd};
use crate::os::Os::Linux;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::{io, thread};
use LinuxType::Ubuntu;
use OsArch::X64;
use PkgType::Deb;

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

/// Notice: Similar to `get_running_processes`, the `process_name_prefix`
/// argument must be a prefix of the actual process name since the low-level
/// commands will probably truncate the name.
pub fn kill_process_and_wait(
    os: Os,
    process_name: &str,
    process_name_prefix: &str,
) -> Result<(), String> {
    kill_process(os, process_name)?;

    // Start the timer
    let start_time = Instant::now();
    let timeout = Duration::from_secs(5);

    // Wait until the process is fully terminated or timeout
    loop {
        let output = Command::new("pgrep")
            .arg(process_name_prefix)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| format!("Fail to start command pgrep: {error}"))?
            .wait_with_output()
            .map_err(|error| format!("Fail to execute command pgrep: {error}"))?;

        // 0 One  or  more processes matched the criteria. For pkill and pid‐
        //   wait, one or more processes must  also  have  been  successfully
        //   signalled or waited for.
        // 1 No processes matched or none of them could be signalled.
        //
        // Source: `man pgrep`
        let ok_status_code = match output.status.code() {
            Some(0) => Ok(0),
            Some(1) => Ok(1),
            Some(code) => Err(format!("Failed to execute pgrep with status code {code}")),
            None => Err("Failed to execute pgrep with no status code".to_string()),
        }?;

        // If pgrep finds no processes, it means the process is terminated
        if ok_status_code == 1 {
            break;
        }

        // Check if the timeout has been reached
        if start_time.elapsed() >= timeout {
            return Err(format!(
                "Process {} (prefix {}) did not terminate within the timeout period.",
                process_name,
                process_name_prefix,
            ));
        }

        // Sleep for a short time before checking again
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
