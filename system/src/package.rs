// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use de::Visitor;
use reqwest::Url;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use VersionError::DigitIntError;
use crate::download::DownloadRequest;
use crate::os::{Os, OsPkg, PkgType};
use crate::package::VersionError::InvalidDigit;

#[derive(Debug)]
pub enum VersionError {
    InvalidDigit(String),
    DigitIntError(ParseIntError),
}

impl Display for VersionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            InvalidDigit(msg) => format!("String has invalid version digits: {msg}"),
            DigitIntError(error) => format!("String contains invalid int digit(s): {error}")
        };

        write!(f, "{}", msg)
    }
}

#[derive(PartialEq, Debug)]
pub struct SemVer(pub u8, pub u8, pub u8);

impl Display for SemVer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl FromStr for SemVer {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_to_version_error = |parse_error: ParseIntError| DigitIntError(parse_error);

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

impl Serialize for SemVer {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

struct SemVerVisitor;

impl<'de> Visitor<'de> for SemVerVisitor {
    type Value = SemVer;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a version string in the format x.y.z")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        SemVer::from_str(v).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for SemVer {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(SemVerVisitor)
    }
}

#[derive(PartialEq, Debug)]
pub struct SemVerRev(pub u8, pub u8, pub u8, pub u16);

impl Display for SemVerRev {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

impl FromStr for SemVerRev {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_to_version_error = |parse_error: ParseIntError| DigitIntError(parse_error);

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

impl Serialize for SemVerRev {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

struct SemVerRevVisitor;

impl<'de> Visitor<'de> for SemVerRevVisitor {
    type Value = SemVerRev;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a version string in the format x.y.z.w")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        SemVerRev::from_str(v).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for SemVerRev {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(SemVerRevVisitor)
    }
}

#[derive(PartialEq, Debug)]
pub struct SemVerVendor(pub u8, pub u8, pub u8, pub String);

impl Display for SemVerVendor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}-{}", self.0, self.1, self.2, self.3)
    }
}

impl FromStr for SemVerVendor {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, '-').collect();

        if parts.len() != 2 {
            return Err(InvalidDigit(format!("String {} must contain a vendor part after the version", s)));
        }

        let version_part = parts[0];
        let vendor_part = parts[1].to_string();
        let SemVer(major, minor, patch) = SemVer::from_str(version_part)?;

        Ok(SemVerVendor(major, minor, patch, vendor_part))
    }
}

impl Serialize for SemVerVendor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

struct SemVerVendorVisitor;

impl<'de> Visitor<'de> for SemVerVendorVisitor {
    type Value = SemVerVendor;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a version string in the format x.y.z-vendor")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        SemVerVendor::from_str(v).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for SemVerVendor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(SemVerVendorVisitor)
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
pub struct Package {
    pub name: String,
    pub software: Software,
    pub os: Os,
    pub doc: Url,
    pub fetch: DownloadRequest,
}

impl Package {
    pub fn new(name: &str,
        os: Os,
        software: Software,
        doc: Url,
        fetch: DownloadRequest,
    ) -> Self {
        Package { name: name.to_string(), os, software, doc, fetch }
    }

    pub fn to_os_pkg(&self, pkg_type: PkgType) -> OsPkg {
        OsPkg { pkg_type, name: self.name.clone() }
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("Package name: {}, Software: {:?}, Documentation: {}, Fetch: {:?}", self.name, self.software, self.doc, self.fetch))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use reqwest::Url;
    use crate::download::{DownloadRequest, Integrity};
    use crate::download::gpg::GpgKey;
    use crate::os::UBUNTU_X64;
    use crate::package::{Package, SemVer, SemVerRev, SemVerVendor, Software};

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
    fn semver_vendor_to_string() {
        let ver = SemVerVendor(21, 0, 1, "amzn".to_string());

        assert_eq!("21.0.1-amzn", ver.to_string())
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
    fn semver_vendor_from_str() {
        let sem_ver_vendor_str = "1.2.3-vendor";
        let sem_ver_vendor = SemVerVendor::from_str(sem_ver_vendor_str).unwrap();

        assert_eq!(sem_ver_vendor, SemVerVendor(1, 2, 3, "vendor".to_string()));
    }

    #[test]
    fn semver_rev_from_str_invalid() {
        let sem_ver_rev_str = "1.2.3";
        let result = SemVerRev::from_str(sem_ver_rev_str);

        assert!(result.is_err());
    }

    #[test]
    fn semver_serialize_to_string() {
        let ver = SemVer(1, 2, 3);
        let ser = serde_json::to_string(&ver)
            .expect("Fail to serialize SemVer to String");

        assert_eq!(format!("\"{}\"", ver.to_string()), ser);
    }

    #[test]
    fn semver_rev_serialize_to_string() {
        let ver = SemVerRev(1, 2, 3, 4);
        let ser = serde_json::to_string(&ver)
            .expect("Fail to serialize SemVerRev to String");

        assert_eq!(format!("\"{}\"", ver.to_string()), ser);
    }

    #[test]
    fn creates_software_model() {
        let version = SemVerRev(6, 1, 1, 443);
        let zoom = Software::new("Zoom Video Communications, Inc", "Zoom", &version.to_string());

        assert_eq!("Zoom Video Communications, Inc", zoom.provider);
        assert_eq!("Zoom", zoom.name);
        assert_eq!("6.1.1.443", zoom.version);
    }

    #[test]
    fn creates_package() {
        let version = SemVerRev(6, 1, 1, 443);
        let zoom = Software::new("Zoom Video Communications, Inc", "Zoom", &version.to_string());
        let os = UBUNTU_X64;
        let fetch_url = "https://zoom.us/client/6.1.1.443/zoom_amd64.deb";
        let gpg_key_url = Url::parse("https://zoom.us/linux/download/pubkey?version=5-12-6").unwrap();
        let gpg_key = GpgKey::new(gpg_key_url, "59C8 6188 E22A BB19 BD55 4047 7B04 A1B8 DD79 B481".to_string());
        let package = Package::new(
            "zoom",
            os,
            zoom,
            Url::parse("https://zoom.us/download").unwrap(),
            DownloadRequest::new(&fetch_url, Integrity::Gpg(gpg_key)).unwrap(),
        );

        assert_eq!("zoom", package.name);
        assert_eq!(UBUNTU_X64, package.os);
    }
}
