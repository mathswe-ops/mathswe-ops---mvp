// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::download::DownloadRequest;
use crate::package::LinuxType::Ubuntu;
use crate::package::Os::Linux;
use crate::package::OsArch::X64;
use crate::package::VersionError::InvalidDigit;

#[derive(Debug)]
pub enum VersionError {
    InvalidDigit(String),
    ParseIntError(ParseIntError),
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct SemVer(pub u8, pub u8, pub u8);

impl Display for SemVer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl FromStr for SemVer {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_to_version_error = |parse_error: ParseIntError| VersionError::ParseIntError(parse_error);

        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 3 {
            return Err(InvalidDigit(format!("String {} must have 3 digits but has {}", s, parts.len())));
        }

        let major = parts[0].parse::<u8>().map_err(parse_to_version_error)?;
        let minor = parts[1].parse::<u8>().map_err(parse_to_version_error)?;
        let patch = parts[2].parse::<u8>().map_err(parse_to_version_error)?;

        Ok(SemVer(major, minor, patch))
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct SemVerRev(pub u8, pub u8, pub u8, pub u16);

impl Display for SemVerRev {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

impl FromStr for SemVerRev {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_to_version_error = |parse_error: ParseIntError| VersionError::ParseIntError(parse_error);

        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 4 {
            return Err(InvalidDigit(format!("String {} must have 4 digits but has {}", s, parts.len())));
        }

        let major = parts[0].parse::<u8>().map_err(parse_to_version_error)?;
        let minor = parts[1].parse::<u8>().map_err(parse_to_version_error)?;
        let patch = parts[2].parse::<u8>().map_err(parse_to_version_error)?;
        let rev = parts[3].parse::<u16>().map_err(parse_to_version_error)?;

        Ok(SemVerRev(major, minor, patch, rev))
    }
}

#[derive(Clone, Debug)]
pub struct Software {
    pub provider: String,
    pub name: String,
    pub version: String,
}

impl Software {
    pub fn new(provider: &str, name: &str, version: &str) -> Self {
        Software { provider: provider.to_string(), name: name.to_string(), version: version.to_string() }
    }
}

#[derive(Clone, Debug)]
pub enum OsArch {
    X64
}

#[derive(Clone, Debug)]
pub enum LinuxType {
    Ubuntu
}

#[derive(Clone, Debug)]
pub enum Os {
    Linux(OsArch, LinuxType)
}

pub const UBUNTU_X64: Os = Linux(X64, Ubuntu);

#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub software: Software,
    pub os: Os,
    pub doc: Url,
    pub fetch: DownloadRequest,
}

impl Package {
    pub fn new(name: &str, os: Os, software: Software, doc: Url, fetch: DownloadRequest) -> Package {
        Package { name: name.to_string(), os, software, doc, fetch }
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("Package name: {}, Software: {:?}, Documentation: {}, Fetch: {:?}", self.name, self.software, self.doc, self.fetch))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::package::{SemVer, SemVerRev};

    #[test]
    fn semver_to_string() {
        let ver = SemVer(2, 10, 6);

        assert_eq!("2.10.6", ver.to_string())
    }

    #[test]
    fn semver_rev_to_string() {
        let ver = SemVerRev(2, 10, 6, 465);

        assert_eq!("2.10.6.465", ver.to_string())
    }

    #[test]
    fn semver_from_str() {
        let sem_ver_str = "1.2.3";
        let sem_ver = SemVer::from_str(sem_ver_str).unwrap();

        assert_eq!(sem_ver, SemVer(1, 2, 3));
    }

    #[test]
    fn semver_from_str_invalid() {
        let sem_ver_str = "1.2";
        let result = SemVer::from_str(sem_ver_str);

        assert!(result.is_err());
    }

    #[test]
    fn semver_rev_from_str() {
        let sem_ver_rev_str = "1.2.3.4";
        let sem_ver_rev = SemVerRev::from_str(sem_ver_rev_str).unwrap();

        assert_eq!(sem_ver_rev, SemVerRev(1, 2, 3, 4));
    }

    #[test]
    fn semver_rev_from_str_invalid() {
        let sem_ver_rev_str = "1.2.3";
        let result = SemVerRev::from_str(sem_ver_rev_str);

        assert!(result.is_err());
    }
}
