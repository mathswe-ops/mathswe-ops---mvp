// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::image::{Image, ImageId, StrFind, ToImageId };
use crate::image::desktop::DesktopImageId::Zoom;
use crate::package::Package;

#[derive(Clone, Debug)]
pub enum DesktopImageId {
    Zoom,
}

impl Display for DesktopImageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Zoom => "zoom",
        };

        write!(f, "{}", msg)
    }
}

impl StrFind for DesktopImageId {
    fn str_find(s: &str) -> Option<Self> {
        match s {
            "zoom" => Some(Zoom),
            _ => None
        }
    }
}

impl FromStr for DesktopImageId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::str_find(s)
            .ok_or_else(|| format!("String {} does not map to DesktopImageId", s))
    }
}

impl ToImageId for DesktopImageId {
    fn to_image_id(&self) -> ImageId {
        ImageId(self.to_string())
    }
}

#[derive(Clone)]
pub struct DesktopImage(DesktopImageId, Package);

impl Display for DesktopImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("Image: {}, Package: {}", self.id(), self.package()))
    }
}

impl Image for DesktopImage {
    fn id(&self) -> ImageId {
        self.0.to_image_id()
    }

    fn package(&self) -> Package {
        self.1.clone()
    }
}

pub mod zoom {
    use reqwest::Url;
    use serde::{Deserialize, Serialize};

    use Os::Linux;

    use crate::download::{DownloadRequest, Integrity};
    use crate::image::{Image, ImageInfoError, ImageInfoLoader, ImageOps, Install, Uninstall};
    use crate::image::desktop::DesktopImage;
    use crate::image::desktop::DesktopImageId::Zoom;
    use crate::package::{Os, Package, SemVerRev, Software};
    use crate::package::LinuxType::Ubuntu;
    use crate::package::OsArch::X64;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ZoomInfo {
        version: SemVerRev,
    }

    pub struct ZoomImage(DesktopImage);

    impl ZoomImage {
        pub fn filename(os: Os) -> String {
            match os {
                Linux(X64, Ubuntu) => "zoom_amd64.deb"
            }.to_string()
        }

        pub fn new(os: Os, ZoomInfo { version }: ZoomInfo) -> ZoomImage {
            let id = Zoom;
            let pkg_id = id.to_string();
            let fetch_url = format!(
                "https://zoom.us/client/{}/{}",
                version,
                Self::filename(os.clone())
            );

            ZoomImage(
                DesktopImage(
                    id,
                    Package::new(
                        &pkg_id,
                        os,
                        Software::new("Zoom Video Communications, Inc", "Zoom", &version.to_string()),
                        Url::parse("https://zoom.us/download").unwrap(),
                        DownloadRequest::new(&fetch_url, Integrity::None).unwrap(),
                    )))
        }

        pub fn from(os: Os, info: ZoomInfo) -> Option<Box<dyn ImageOps>> {
            Some(Box::new(Self::new(os, info)))
        }

        pub fn load_with(os: Os, info_loader: ImageInfoLoader) -> Result<Option<Box<dyn ImageOps>>, ImageInfoError> {
            let info = info_loader.load(Zoom)?;
            Ok(Self::from(os, info))
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

    impl ImageOps for ZoomImage {
        fn image(&self) -> Box<dyn Image> {
            Box::new(self.0.clone())
        }
    }

    #[cfg(test)]
    mod tests {
        use std::path::PathBuf;

        use crate::download::Integrity;
        use crate::image::desktop::DesktopImage;
        use crate::image::desktop::DesktopImageId::Zoom;
        use crate::image::desktop::zoom::{ZoomImage, ZoomInfo};
        use crate::image::ImageInfoLoader;
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
            let ZoomImage(DesktopImage(id, package)) = ZoomImage::new(UBUNTU_X64, zoom_info);

            assert_eq!("zoom", id.to_string());
            assert_eq!("zoom", package.name);
            assert_eq!("Zoom", package.software.name);
            assert_eq!("6.1.1.443", package.software.version);
            assert_eq!("https://zoom.us/client/6.1.1.443/zoom_amd64.deb", package.fetch.url().as_str());
            assert_eq!(Integrity::None, package.fetch.integrity());
            todo!("must implement GPG")
        }
    }
}
