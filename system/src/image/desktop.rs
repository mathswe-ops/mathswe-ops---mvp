// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use core::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::image::{Image, ImageId, StrFind, ToImageId};
use crate::image::desktop::DesktopImageId::Zoom;
use crate::impl_image;
use crate::package::Package;

#[derive(Clone, Debug)]
pub enum DesktopImageId {
    Zoom,
}

impl Display for DesktopImageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

impl_image!(DesktopImage);

pub mod zoom {
    use reqwest::Url;
    use serde::{Deserialize, Serialize};

    use crate::cmd::exec_cmd;
    use crate::download::{Downloader, DownloadRequest, Integrity};
    use crate::download::gpg::GpgKey;
    use crate::image::{Image, ImageOps, Install, Uninstall};
    use crate::image::desktop::DesktopImage;
    use crate::image::desktop::DesktopImageId::Zoom;
    use crate::image_ops_impl;
    use crate::os::LinuxType::Ubuntu;
    use crate::os::Os;
    use crate::os::Os::Linux;
    use crate::os::OsArch::X64;
    use crate::os::PkgType::Deb;
    use crate::package::{Package, SemVerRev, Software};
    use crate::tmp::TmpWorkingDir;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ZoomInfo {
        version: SemVerRev,
        public_key_version: String,
        key_fingerprint: String,
    }

    pub struct ZoomImage(DesktopImage);

    impl ZoomImage {
        pub fn new(
            os: Os,
            ZoomInfo { version, public_key_version, key_fingerprint }: ZoomInfo,
        ) -> Self {
            let id = Zoom;
            let pkg_id = id.to_string();
            let filename = match os {
                Linux(X64, Ubuntu) => "zoom_amd64.deb"
            };
            let fetch_url = format!("https://zoom.us/client/{}/{}", version, filename);
            let gpg_key_url = Url::parse(format!("https://zoom.us/linux/download/pubkey?version={}", public_key_version).as_str()).unwrap();
            let gpg_key = GpgKey::new(gpg_key_url, key_fingerprint);

            ZoomImage(
                DesktopImage(
                    id,
                    Package::new(
                        &pkg_id,
                        os,
                        Software::new("Zoom Video Communications, Inc", "Zoom", &version.to_string()),
                        Url::parse("https://zoom.us/download").unwrap(),
                        DownloadRequest::new(&fetch_url, Integrity::Gpg(gpg_key)).unwrap(),
                    )))
        }
    }

    impl Install for ZoomImage {
        fn install(&self) -> Result<(), String> {
            let package = self.0.package();
            let tmp = TmpWorkingDir::new()
                .map_err(|error| error.to_string())?;

            let downloader = Downloader::from(package.fetch.clone(), &tmp);
            let file_path = downloader.path.clone();

            println!("Downloading Zoom...");

            downloader
                .download_blocking()
                .map_err(|error| error.to_string())?;

            println!("Installing Zoom...");

            package
                .to_os_pkg(Deb)
                .install(&file_path)?;

            println!("Installing unmet dependencies...");

            let output = exec_cmd(
                "sudo",
                &["apt-get", "--fix-broken", "--yes", "install"],
            ).map_err(|error| error.to_string())?;
            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            Ok(())
        }
    }

    impl Uninstall for ZoomImage {
        fn uninstall(&self) -> Result<(), String> {
            self.0.package().to_os_pkg(Deb).uninstall()
        }
    }

    impl ImageOps for ZoomImage { image_ops_impl!(); }

    #[cfg(test)]
    mod tests {
        use std::path::PathBuf;

        use reqwest::Url;

        use crate::download::gpg::GpgKey;
        use crate::download::Integrity;
        use crate::image::desktop::DesktopImage;
        use crate::image::desktop::DesktopImageId::Zoom;
        use crate::image::desktop::zoom::{ZoomImage, ZoomInfo};
        use crate::image::ImageInfoLoader;
        use crate::os::UBUNTU_X64;
        use crate::package::SemVerRev;

        #[test]
        fn loads_zoom_image_info() {
            let info = ImageInfoLoader::from(
                &Zoom,
                PathBuf::from("resources/test/image"),
                PathBuf::from(""),
            );
            let zoom_info: ZoomInfo = info
                .load()
                .expect("Fail to load Zoom test image");

            assert_eq!("6.1.1.443", zoom_info.version.to_string())
        }

        #[test]
        fn creates_zoom_image() {
            let zoom_info = ZoomInfo {
                version: SemVerRev(6, 1, 1, 443),
                public_key_version: "5-12-6".to_string(),
                key_fingerprint: "59C8 6188 E22A BB19 BD55 4047 7B04 A1B8 DD79 B481".to_string(),
            };
            let ZoomImage(DesktopImage(id, package)) = ZoomImage::new(UBUNTU_X64, zoom_info);
            let expected_gpg_key = GpgKey::new(
                Url::parse("https://zoom.us/linux/download/pubkey?version=5-12-6").unwrap(),
                "59C8 6188 E22A BB19 BD55 4047 7B04 A1B8 DD79 B481".to_string(),
            );

            assert_eq!("zoom", id.to_string());
            assert_eq!("zoom", package.name);
            assert_eq!("Zoom", package.software.name);
            assert_eq!("6.1.1.443", package.software.version);
            assert_eq!("https://zoom.us/client/6.1.1.443/zoom_amd64.deb", package.fetch.url().as_str());
            assert_eq!(Integrity::Gpg(expected_gpg_key), package.fetch.integrity());
        }
    }
}
