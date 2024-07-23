// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use reqwest::Url;
use crate::download::DownloadRequest;
use crate::package::LinuxType::Ubuntu;
use crate::package::Os::Linux;
use crate::package::OsArch::X64;

#[derive(PartialEq)]
pub struct SemVer(pub u8, pub u8, pub u8);

impl Display for SemVer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

#[derive(PartialEq)]
pub struct SemVerRev(pub u8, pub u8, pub u8, pub u16);

impl Display for SemVerRev {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
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
}
