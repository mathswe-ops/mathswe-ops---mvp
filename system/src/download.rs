// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use reqwest::{blocking, Url};

use DownloadRequestError::{InsecureProtocol, InvalidUrl};

use crate::download::hashing::Hash;

pub mod hashing;
mod gpg;

#[derive(PartialEq, Clone, Debug)]
pub enum Integrity {
    Hash(Hash),
    None,
}

impl Integrity {
    pub fn check(&self, file_path: &Path) -> io::Result<bool> {
        match self {
            Integrity::Hash(hash) => hash.matches(file_path),
            Integrity::None => Ok(true),
        }
    }
}

#[derive(Debug)]
pub enum DownloadRequestError {
    InvalidUrl { url: String, error: String },
    InsecureProtocol { url: String },
}

impl Display for DownloadRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            InvalidUrl { url, error } => format!("Invalid URL {}. Cause: {}", url, error),
            InsecureProtocol { url } => format!("URL {} protocol is not HTTPS", url),
        };

        write!(f, "{}", msg)
    }
}

#[derive(Clone, Debug)]
pub struct DownloadRequest {
    url: Url,
    integrity: Integrity,
}

impl DownloadRequest {
    pub fn new(url_raw: &str, integrity: Integrity) -> Result<Self, DownloadRequestError> {
        Ok(url_raw)
            .and_then(Url::parse)
            .map_err(|error| InvalidUrl { url: url_raw.to_string(), error: error.to_string() })
            .and_then(|url| {
                if url.scheme() == "https" {
                    Ok(DownloadRequest { url, integrity })
                } else {
                    Err(InsecureProtocol { url: url.to_string() })
                }
            })
    }

    pub fn url(&self) -> Url {
        self.url.clone()
    }

    pub fn integrity(&self) -> Integrity {
        self.integrity.clone()
    }
}

pub struct Downloader {
    req: DownloadRequest,
    path: PathBuf,
}

impl Downloader {
    pub fn new(req: DownloadRequest, path: PathBuf) -> Self {
        Downloader { req, path }
    }

    pub fn to_file(&self) -> io::Result<File> {
        File::create_new(&self.path)
    }

    pub fn download_blocking(&self, filename: &str) -> io::Result<()> {
        let format_err_msg = |msg: String, target: String| { format!("{}: {}", msg, target) };

        let io_err = |msg: String| { io::Error::new(ErrorKind::Other, msg) };

        let to_io_err = |msg: String| |err: reqwest::Error| io_err(format_err_msg(msg, err.to_string()));

        let url = &self.req.url;

        blocking::get(url.clone())
            .map_err(to_io_err(format!("Failed to fetch {}", url)))
            .and_then(|res| {
                if res.status().is_success() {
                    res
                        .bytes()
                        .map_err(|err| io_err(format!("Failed to read file bytes {}: {}", filename, err)))
                } else {
                    Err(io_err(format!("Failed to download {}: {}", filename, res.status())))
                }
            })
            .and_then(|bytes| {
                self.to_file()
                    .and_then(|mut file| file.write_all(&bytes))
                    .map_err(|err| io_err(format!("Failed to write {}: {}", filename, err)))
            })
            .and_then(|_| {
                self.req
                    .integrity
                    .check(self.path.as_path())
                    .and_then(|check| {
                        if check {
                            Ok(())
                        } else {
                            Err(io_err(format!("Downloaded file {} failed {:?} integrity check", filename, self.req.integrity)))
                        }
                    })
            })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use crate::download::hashing::HashAlgorithm;
    use crate::tmp::TmpWorkingDir;

    use super::*;

    #[test]
    fn checks_url() {
        let req = DownloadRequest::new(
            "https://example.com",
            Integrity::None,
        );

        assert!(req.is_ok());
    }

    #[test]
    fn rejects_unsafe_url() {
        let req = DownloadRequest::new(
            "http://example.com",
            Integrity::None,
        );

        assert!(req.is_err());

        let error_matches = match req {
            Err(InsecureProtocol { url }) => url == "http://example.com/",
            _ => false
        };
        assert!(error_matches);
    }

    #[test]
    fn downloads_file() -> io::Result<()> {
        let base_url = "https://raw.githubusercontent.com/mathswe-ops/mathswe-ops---mvp/main";
        let filename = "test_file.txt";
        let url = format!("{}/system/resources/test/download/{}", base_url, filename);
        let temp_dir = TmpWorkingDir::new()?;
        let temp_file_path = temp_dir.join(filename.as_ref());
        let checksum = "0ecfebe350c45dbded8cfb32d3af0b910bde66fc2aafbafabdaaeef6cae48a59".to_string();
        let integrity = Integrity::Hash(Hash::new(HashAlgorithm::Sha256, checksum));
        let req = DownloadRequest::new(&url, integrity)
            .expect("Fail to build a correct download request");

        let downloader = Downloader::new(req, temp_file_path.clone());

        downloader.download_blocking(filename)?;
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
        let req = DownloadRequest::new(&url, Integrity::None)
            .expect("Fail to build a correct download request");

        let downloader = Downloader::new(req, temp_file_path.clone());

        let res = downloader.download_blocking(filename);

        match res {
            Ok(_) => { panic!("It could download non-existent file!") }
            Err(err) => { assert!(err.to_string().contains(": 404 Not Found")) }
        }
        assert_eq!(temp_file_path.exists(), false);

        Ok(())
    }
}
