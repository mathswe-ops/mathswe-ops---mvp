// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::str::FromStr;

use clap::ValueEnum;
use zoom::ZoomImage;
use crate::image::{Install, Uninstall};
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

pub fn load_image(id: ImageId, os: Os) -> Option<impl Image> {
    match id {
        Zoom => ZoomImage::from(os, zoom::DEF_VERSION)
    }
}

mod zoom {
    use std::fmt::{Display, Formatter};

    use reqwest::Url;

    use Os::Linux;

    use crate::download::{DownloadRequest, Integrity};
    use crate::image::{Install, Uninstall};
    use crate::image::images::{Image, ImageId};
    use crate::image::images::ImageId::Zoom;
    use crate::package::{Os, Package, SemVerRev, Software};
    use crate::package::LinuxType::Ubuntu;
    use crate::package::OsArch::X64;

    pub struct ZoomImage(Package);

    impl ZoomImage {
        pub fn filename(os: Os) -> String {
            match os {
                Linux(X64, Ubuntu) => "zoom_amd64.deb"
            }.to_string()
        }

        pub fn from(os: Os, version: SemVerRev) -> Option<Self> {
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

    // It will be deprecated when reading this volatile data from system/package/zoom config file...
    pub const DEF_VERSION: SemVerRev = SemVerRev(6, 1, 1, 443);

    #[cfg(test)]
    mod tests {
        use crate::download::Integrity;
        use crate::image::images::zoom::ZoomImage;
        use crate::package::{SemVerRev, UBUNTU_X64};

        #[test]
        fn creates_zoom_image() {
            let ZoomImage(package) = ZoomImage::from(UBUNTU_X64, SemVerRev(6, 1, 1, 443))
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
