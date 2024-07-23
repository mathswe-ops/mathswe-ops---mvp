// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use clap::ValueEnum;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use zoom::ZoomImage;

use crate::image::{ImageInfoError, ImageInfoLoader, Install, Uninstall};
use crate::image::images::ImageId::Zoom;
use crate::package::{Os, Package};

#[derive(Clone, Debug, ValueEnum)]
pub enum ImageId {
    Zoom
}

impl Display for ImageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Zoom => "zoom"
        };

        write!(f, "{}", msg)
    }
}

pub trait Image: Display + Install + Uninstall {
    const ID: ImageId;

    fn package(&self) -> Package;

    fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("Image: {}, Package: {}", Self::ID, self.package()))
    }
}

pub fn load_image(id: ImageId, os: Os) -> Result<Option<impl Image>, ImageInfoError> {
    let info = ImageInfoLoader { root: PathBuf::from("image"), dir: PathBuf::from("") };
    let image = match id {
        Zoom => info.load(id).map(|info| ZoomImage::from(os, info))?
    };

    Ok(image)
}

mod zoom {
    use std::fmt::{Display, Formatter};
    use std::str::FromStr;

    use reqwest::Url;
    use serde::{Deserialize, Serialize};

    use Os::Linux;

    use crate::download::{DownloadRequest, Integrity};
    use crate::image::{Install, Uninstall};
    use crate::image::images::{Image, ImageId};
    use crate::image::images::ImageId::Zoom;
    use crate::package::{Os, Package, SemVerRev, Software};
    use crate::package::LinuxType::Ubuntu;
    use crate::package::OsArch::X64;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ZoomInfo {
        version: SemVerRev,
    }

    pub struct ZoomImage(Package);

    impl ZoomImage {
        pub fn filename(os: Os) -> String {
            match os {
                Linux(X64, Ubuntu) => "zoom_amd64.deb"
            }.to_string()
        }

        pub fn from(os: Os, ZoomInfo { version }: ZoomInfo) -> Option<Self> {
            let fetch_url = format!(
                "https://zoom.us/client/{}/{}",
                version,
                Self::filename(os.clone())
            );

            Some(ZoomImage(
                Package::new(
                    &Self::ID.to_string(),
                    os,
                    Software::new("Zoom Video Communications, Inc", "Zoom", &version.to_string()),
                    Url::parse("https://zoom.us/download").unwrap(),
                    DownloadRequest::new(&fetch_url, Integrity::None).unwrap(),
                )))
        }
    }

    impl Image for ZoomImage {
        const ID: ImageId = Zoom;

        fn package(&self) -> Package {
            self.0.clone()
        }
    }

    impl Display for ZoomImage {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.format(f)
        }
    }

    impl Install for ZoomImage {
        fn install(&self) -> Result<(), String> {
            todo!()
        }
    }

    impl Uninstall for ZoomImage {
        fn uninstall(&self) -> Result<(), String> {
            todo!()
        }
    }

    #[cfg(test)]
    mod tests {
        use std::path::PathBuf;

        use crate::download::Integrity;
        use crate::image::images::ImageId::Zoom;
        use crate::image::images::ImageInfoLoader;
        use crate::image::images::zoom::{ZoomImage, ZoomInfo};
        use crate::package::{SemVerRev, UBUNTU_X64};

        #[test]
        fn loads_zoom_image_info() {
            let info = ImageInfoLoader {
                root: PathBuf::from("resources/test/image"),
                dir: PathBuf::from(""),
            };
            let zoom_info: ZoomInfo = info
                .load(Zoom)
                .expect("Fail to load Zoom test image");

            assert_eq!("6.1.1.443", zoom_info.version.to_string())
        }

        #[test]
        fn creates_zoom_image() {
            let zoom_info = ZoomInfo { version: SemVerRev(6, 1, 1, 443) };
            let ZoomImage(package) = ZoomImage::from(UBUNTU_X64, zoom_info)
                .unwrap();

            assert_eq!("zoom", package.name);
            assert_eq!("Zoom", package.software.name);
            assert_eq!("6.1.1.443", package.software.version);
            assert_eq!("https://zoom.us/client/6.1.1.443/zoom_amd64.deb", package.fetch.url().as_str());
            assert_eq!(Integrity::None, package.fetch.integrity());
            todo!("must implement GPG")
        }
    }
}
