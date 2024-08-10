// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use core::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use DesktopImageId::VsCode;

use crate::image::{Image, ImageId, StrFind, ToImageId};
use crate::image::desktop::DesktopImageId::Zoom;
use crate::impl_image;
use crate::package::Package;

#[derive(PartialEq, Clone, Debug)]
pub enum DesktopImageId {
    Zoom,
    VsCode,
}

impl Display for DesktopImageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Zoom => "zoom",
            VsCode => "vscode",
        };

        write!(f, "{}", msg)
    }
}

impl StrFind for DesktopImageId {
    fn str_find(s: &str) -> Option<Self> {
        match s {
            "zoom" => Some(Zoom),
            "vscode" => Some(VsCode),
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

pub mod vscode {
    use reqwest::{blocking, Url};
    use reqwest::redirect::Policy;
    use serde::{Deserialize, Serialize};

    use Os::Linux;

    use crate::download::{Downloader, DownloadRequest, Integrity};
    use crate::download::hashing::Hash;
    use crate::download::hashing::HashAlgorithm::Sha256;
    use crate::image::{Image, ImageOps, Install, Uninstall};
    use crate::image::desktop::DesktopImage;
    use crate::image::desktop::DesktopImageId::VsCode;
    use crate::image_ops_impl;
    use crate::os::Os;
    use crate::os::OsArch::X64;
    use crate::os::PkgType::Deb;
    use crate::package::{Package, SemVer, Software};
    use crate::tmp::TmpWorkingDir;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct VsCodeInfo {
        version: SemVer,
        hash_sha256: String,
        use_latest_if_version_is_old: bool,
    }

    pub struct VsCodeImage(DesktopImage, VsCodeInfo);

    impl VsCodeImage {
        pub fn new(os: Os, info: VsCodeInfo) -> Self {
            let VsCodeInfo { version, hash_sha256, .. } = info.clone();
            let id = VsCode;
            let pkg_name = "code";
            let fetch_url = match os {
                Linux(X64, _) => "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64",
            };
            let hash = Hash::new(Sha256, hash_sha256);

            VsCodeImage(DesktopImage(
                id,
                Package::new(
                    pkg_name,
                    os,
                    Software::new("Microsoft Corporation", "Visual Studio Code", &version.to_string()),
                    Url::parse("https://code.visualstudio.com/download").unwrap(),
                    DownloadRequest::new(fetch_url, Integrity::Hash(hash)).unwrap(),
                ),
            ), info)
        }

        /// The original fetch URL is generic for the `latest` version, so the
        /// link redirects to a new low-level URL with the actual app version
        /// and direct download. The program should download from the actual URL
        /// to check the expected version (VsCodeInfo) hash correctly.
        fn get_actual_download_request(&self) -> Result<DownloadRequest, String> {
            let final_url = blocking::Client::builder()
                .redirect(Policy::limited(10))
                .build()
                .map_err(|error| error.to_string())?
                .head(self.0.package().fetch.url())
                .send()
                .map_err(|error| error.to_string())?
                .url()
                .clone();

            let package = self.0.package();
            let original_fetch = package.fetch;
            let original_version = package.software.version;
            let expected_name = format!("/code_{original_version}");

            if final_url.to_string().contains(&expected_name) {
                let actual_req = DownloadRequest::new(
                    &final_url.to_string(),
                    original_fetch.integrity(),
                ).map_err(|error| error.to_string())?;

                Ok(actual_req)
            } else if self.1.use_latest_if_version_is_old {
                let actual_req = DownloadRequest::new(
                    &final_url.to_string(),
                    Integrity::None,
                ).map_err(|error| error.to_string())?;

                println!("Unable to fetch version {}.", original_version);
                println!("Fetching the latest version without hash integrity check since use_latest_if_version_is_old is true.");

                Ok(actual_req)
            } else {
                let msg = format!("Unable to fetch required version {original_version}.");

                eprintln!("{}", msg);
                println!("Redirect URL: {final_url}.");
                println!("Hint: Make sure to update the vscode.json to the latest version or set use_latest_if_version_is_old to true.");

                Err(msg)
            }
        }
    }

    impl Install for VsCodeImage {
        fn install(&self) -> Result<(), String> {
            let tmp = TmpWorkingDir::new()
                .map_err(|error| error.to_string())?;

            let req = self.get_actual_download_request()
                          .map_err(|error| error.to_string())?;

            let downloader = Downloader::from(req, &tmp);
            let installer_file = downloader.path.clone();

            println!("Downloading Visual Studio Code installer...");

            downloader
                .download_blocking()
                .map_err(|error| error.to_string())?;

            println!("Installing Visual Studio Code...");

            self.0.package().to_os_pkg(Deb).install(&installer_file)?;

            println!("Visual Studio Code installed.");

            Ok(())
        }
    }

    impl Uninstall for VsCodeImage {
        fn uninstall(&self) -> Result<(), String> {
            println!("Uninstalling Visual Studio Code...");

            self.0.package().to_os_pkg(Deb).uninstall()?;

            println!("Visual Studio Code uninstalled.");

            Ok(())
        }
    }

    impl ImageOps for VsCodeImage { image_ops_impl!(); }

    #[cfg(test)]
    mod tests {
        use std::str::FromStr;
        use crate::image::desktop::DesktopImageId::VsCode;
        use crate::image::desktop::vscode::{VsCodeImage, VsCodeInfo};
        use crate::image::{Image, ToImageId};
        use crate::image::desktop::DesktopImageId;
        use crate::os::UBUNTU_X64;
        use crate::package::SemVer;

        fn dummy_info() -> VsCodeInfo {
            VsCodeInfo {
                version: SemVer(1, 92, 1),
                hash_sha256: "d0f161ec79145772445d5a14b15030592498aaafa59237a602d66f43653e5309".to_string(),
                use_latest_if_version_is_old: true,
            }
        }

        #[test]
        fn uses_correct_high_level_id_name() {
            let id = DesktopImageId::from_str("vscode");

            assert_eq!(Ok(VsCode), id);

            let info = dummy_info();
            let VsCodeImage(image, _) = VsCodeImage::new(UBUNTU_X64, info);

            assert_eq!("vscode".to_string(), image.id().to_string());
        }

        #[test]
        fn uses_correct_low_level_package_name() {
            let info = dummy_info();
            let VsCodeImage(image, _) = VsCodeImage::new(UBUNTU_X64, info);

            assert_eq!(VsCode.to_image_id(), image.id());

            // The low-level package name is "code" not "vscode"
            assert_eq!("code", image.package().name);
        }
    }
}
