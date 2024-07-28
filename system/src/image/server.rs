// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::fmt;
use std::str::FromStr;

use ServerImageId::Rust;

use crate::image::{Image, ImageId, StrFind, ToImageId};
use crate::impl_image;
use crate::package::Package;

#[derive(Clone, Debug)]
pub enum ServerImageId {
    Rust,
}

impl Display for ServerImageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Rust => "rust",
        };

        write!(f, "{}", msg)
    }
}

impl StrFind for ServerImageId {
    fn str_find(s: &str) -> Option<Self> {
        match s {
            "rust" => Some(Rust),
            _ => None
        }
    }
}

impl FromStr for ServerImageId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::str_find(s)
            .ok_or_else(|| format!("String {} does not map to ServerImageId", s))
    }
}

impl ToImageId for ServerImageId {
    fn to_image_id(&self) -> ImageId {
        ImageId(self.to_string())
    }
}

#[derive(Clone)]
pub struct ServerImage(ServerImageId, Package);

impl_image!(ServerImage);

pub mod rust {
    use reqwest::Url;

    use Os::Linux;

    use crate::download::{DownloadRequest, Integrity};
    use crate::image::{Image, ImageOps, Install, Uninstall};
    use crate::image::server::ServerImage;
    use crate::image::server::ServerImageId::Rust;
    use crate::image_ops_impl;
    use crate::package::{Os, Package, Software};

    pub struct RustImage(ServerImage);

    impl RustImage {
        pub fn fetch_url(os: Os) -> String {
            match os {
                Linux(_, _) => "https://sh.rustup.rs"
            }.to_string()
        }

        pub fn new(os: Os) -> RustImage {
            let id = Rust;
            let pkg_id = id.to_string();
            let fetch_url = Self::fetch_url(os.clone());
            let version = "latest";

            RustImage(
                ServerImage(
                    id,
                    Package::new(
                        &pkg_id,
                        os,
                        Software::new("Rust Team", "Rust", version),
                        Url::parse("https://www.rust-lang.org/tools/install").unwrap(),
                        DownloadRequest::new(&fetch_url, Integrity::None).unwrap(),
                    )))
        }

        pub fn from(os: Os) -> Option<Box<dyn ImageOps>> {
            Some(Box::new(Self::new(os)))
        }
    }

    impl Install for RustImage {
        fn install(&self) -> Result<(), String> {
            todo!()
        }
    }

    impl Uninstall for RustImage {
        fn uninstall(&self) -> Result<(), String> {
            todo!()
        }
    }

    impl ImageOps for RustImage { image_ops_impl!(); }
}
