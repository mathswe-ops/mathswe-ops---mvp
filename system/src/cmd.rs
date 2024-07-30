// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::io::Error;
use std::process::{Child, Command, Output, Stdio};

use CmdErrorCause::UnsuccessfulStatus;

use crate::cmd::CmdErrorCause::Io;
use crate::cmd::IoErrorCause::{StartFail, WaitFail};

#[derive(Debug)]
pub enum IoErrorCause { StartFail, WaitFail }

#[derive(Debug)]
pub enum CmdErrorCause { Io(IoErrorCause, Error), UnsuccessfulStatus(Option<i32>) }

#[derive(Debug)]
pub struct CmdError {
    cmd: String,
    cause: CmdErrorCause,
}

impl CmdError {
    fn from(cmd: &str, cause: CmdErrorCause) -> CmdError {
        CmdError {
            cmd: cmd.to_string(),
            cause,
        }
    }
}

impl Display for CmdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let reason = match &self.cause {
            Io(StartFail, err) => format!("Fail to start command {}. \nCause: {}", self.cmd, err),
            Io(WaitFail, err) => format!("Fail to wait for command {} exit. \nCause: {}", self.cmd, err),
            UnsuccessfulStatus(code) => format!("Unsuccessful command {} execution. \nCause: Status code {:?}", self.cmd, code),
        };

        write!(f, "{}", reason)
    }
}

pub type Result<T> = std::result::Result<T, CmdError>;

pub fn exec_cmd(cmd: &str, args: &[&str]) -> Result<Output> {
    let io_err = move |cause: IoErrorCause| move |err: Error| CmdError::from(cmd, Io(cause, err));
    let err = |cause: CmdErrorCause| CmdError::from(cmd, cause);
    let check_success = |output: Output| {
        if output.status.success() {
            Ok(output)
        } else {
            Err(err(UnsuccessfulStatus(output.status.code())))
        }
    };
    let wait_child = |child: Child| {
        child
            .wait_with_output()
            .map_err(io_err(WaitFail))
            .and_then(check_success)
    };

    Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(io_err(StartFail))
        .and_then(wait_child)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_exec_success(cmd: &str, args: &[&str]) {
        println!("{}", format!("Command {} {:?}", cmd, args));

        match exec_cmd(cmd, args) {
            Ok(output) => {
                assert!(output.status.success());
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
            Err(e) => panic!("Command failed: {}", e),
        }
        println!();
    }

    fn assert_exec_status_fail(cmd: &str, args: &[&str], code: i32) {
        println!("{}", format!("Command {} {:?}", cmd, args));

        match exec_cmd(cmd, args) {
            Ok(_) => panic!("Expected command to fail with unsuccessful status code, but it succeeded."),
            Err(e) => {
                let status_code_matches = match e.cause {
                    UnsuccessfulStatus(Some(actual)) => actual == code,
                    _ => false
                };

                assert!(status_code_matches);
            }
        }
        println!();
    }

    fn assert_exec_fail(cmd: &str, args: &[&str]) {
        println!("{}", format!("Command {} {:?}", cmd, args));

        match exec_cmd(cmd, args) {
            Ok(_) => panic!("Expected command to fail, but it succeeded."),
            Err(e) => println!("Command failed as expected: {}", e),
        }
        println!();
    }

    #[test]
    fn execute_ls() {
        assert_exec_success("ls", &["."]);
        assert_exec_status_fail("ls", &["non_existent_directory"], 2);
    }

    #[test]
    fn execute_invalid_command() {
        assert_exec_fail("invalid_command", &[]);
    }

    #[test]
    fn execute_echo() {
        assert_exec_success("echo", &["Hello, world!"]);
    }

    #[test]
    fn execute_fail() {
        assert_exec_fail("non_existent_command", &["non_existent_directory"]);
    }

    #[test]
    fn reads_git_status() -> Result<()> {
        println!();
        println!("Reading Git status");

        exec_cmd("git", &["status"])
            .map(|output| println!("{:?}", output))
    }

    #[test]
    fn downloads_file_with_bash() -> Result<()> {
        let base_url = "https://raw.githubusercontent.com/mathswe-ops/mathswe-ops---mvp/main";
        let filename = "test_file.txt";
        let url = format!("{}/system/resources/test/download/{}", base_url, filename);
        let bash_cmd = format!("curl -sSL {} | cat", url);

        println!();
        println!("Downloading file and printing it");

        let result = exec_cmd("bash", &["-c", &bash_cmd])
            .map(|output| println!("{:?}", output));

        println!("File processed");

        result
    }
}
