// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

use reqwest::blocking;

pub struct Downloader {
    path: PathBuf,
}

impl Downloader {
    pub fn new(path: PathBuf) -> Self {
        Downloader { path }
    }

    pub fn to_file(self) -> io::Result<File> {
        File::create_new(self.path)
    }

    pub fn download_blocking(self, filename: &str, url: &str) -> io::Result<()> {
        let format_err_msg = |msg: String, target: String| { format!("{}: {}", msg, target) };

        let io_err = |msg: String| { io::Error::new(ErrorKind::Other, msg) };

        let to_io_err = |msg: String| |err: reqwest::Error| io_err(format_err_msg(msg, err.to_string()));

        blocking::get(url)
            .map_err(to_io_err(format!("Failed to fetch {}", url)))
            .and_then(|res| {
                if res.status().is_success() {
                    res
                        .bytes()
                        .map_err(|err| io_err(format!("Failed to read file bytes {}", filename)))
                } else {
                    Err(io_err(format!("Failed to download {}: {}", filename, res.status())))
                }
            })
            .and_then(|bytes| {
                self
                    .to_file()
                    .and_then(|mut file| file.write_all(&bytes))
                    .map_err(|err| io_err(format!("Failed to write {}: {}", filename, err)))
            })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use crate::tmp::tmp::TmpWorkingDir;

    use super::*;

    #[test]
    fn downloads_file() -> io::Result<()> {
        let base_url = "https://raw.githubusercontent.com/mathswe-ops/mathswe-ops---mvp/main";
        let filename = "test_file.txt";
        let url = format!("{}/system/resources/test/download/{}", base_url, filename);
        let temp_dir = TmpWorkingDir::new()?;
        let temp_file_path = temp_dir.join(filename.as_ref());
        let downloader = Downloader::new(temp_file_path.clone());

        downloader.download_blocking(filename, &url)?;
        assert!(temp_file_path.exists());

        let test_file_path = Path::new("resources")
            .join("test")
            .join("download")
            .join("test_file.txt");
        let expected_content = fs::read_to_string(test_file_path)?;
        let file_content = fs::read_to_string(temp_file_path)?;

        assert_eq!(expected_content, file_content);
        println!("{} downloaded successfully.", filename);

        Ok(())
    }

    #[test]
    fn fails_with_bad_url() -> io::Result<()> {
        let base_url = "https://raw.githubusercontent.com/mathswe-ops/mathswe-ops---mvp/main";
        let filename = "not-exists.txt";
        let url = format!("{}/system/resources/test/download/{}", base_url, filename);
        let temp_dir = TmpWorkingDir::new()?;
        let temp_file_path = temp_dir.join(filename.as_ref());
        let downloader = Downloader::new(temp_file_path.clone());

        let res = downloader.download_blocking(filename, &url);

        match res {
            Ok(_) => { panic!("It could download non-existent file!") }
            Err(err) => { assert!(err.to_string().contains(": 404 Not Found")) }
        }
        assert_eq!(temp_file_path.exists(), false);

        Ok(())
    }
}
