// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use core::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use DesktopImageId::{JetBrainsToolbox, PyCharm, VsCode};

use crate::image::desktop::DesktopImageId::Zoom;
use crate::image::{Image, ImageId, StrFind, ToImageId};
use crate::impl_image;
use crate::package::Package;

#[derive(PartialEq, Clone, Debug)]
pub enum DesktopImageId {
    Zoom,
    VsCode,
    JetBrainsToolbox,
    PyCharm,
}

impl Display for DesktopImageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Zoom => "zoom",
            VsCode => "vscode",
            JetBrainsToolbox => "jetbrains-toolbox",
            PyCharm => "pycharm",
        };

        write!(f, "{}", msg)
    }
}

impl StrFind for DesktopImageId {
    fn str_find(s: &str) -> Option<Self> {
        match s {
            "zoom" => Some(Zoom),
            "vscode" => Some(VsCode),
            "jetbrains-toolbox" => Some(JetBrainsToolbox),
            "pycharm" => Some(PyCharm),
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
    use crate::download::gpg::GpgKey;
    use crate::download::{DownloadRequest, Downloader, Integrity};
    use crate::image::desktop::DesktopImage;
    use crate::image::desktop::DesktopImageId::Zoom;
    use crate::image::{Image, ImageOps, Install, Uninstall};
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
        use crate::image::desktop::zoom::{ZoomImage, ZoomInfo};
        use crate::image::desktop::DesktopImage;
        use crate::image::desktop::DesktopImageId::Zoom;
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
    use reqwest::redirect::Policy;
    use reqwest::{blocking, Url};
    use serde::{Deserialize, Serialize};

    use Os::Linux;

    use crate::download::hashing::Hash;
    use crate::download::hashing::HashAlgorithm::Sha256;
    use crate::download::{DownloadRequest, Downloader, Integrity};
    use crate::image::desktop::DesktopImage;
    use crate::image::desktop::DesktopImageId::VsCode;
    use crate::image::{Image, ImageOps, Install, Uninstall};
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

        use crate::image::desktop::vscode::{VsCodeImage, VsCodeInfo};
        use crate::image::desktop::DesktopImageId;
        use crate::image::desktop::DesktopImageId::VsCode;
        use crate::image::{Image, ToImageId};
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

pub mod jetbrains_toolbox {
    use std::path::PathBuf;
    use std::{env, fs};
    use std::process::Output;
    use reqwest::Url;
    use serde::{Deserialize, Serialize};
    use Os::Linux;

    use crate::cmd::exec_cmd;
    use crate::download::hashing::Hash;
    use crate::download::hashing::HashAlgorithm::Sha256;
    use crate::download::{DownloadRequest, Downloader, Integrity};
    use crate::image::desktop::DesktopImage;
    use crate::image::desktop::DesktopImageId::JetBrainsToolbox;
    use crate::image::Image;
    use crate::image::{ImageOps, Install, Uninstall};
    use crate::image_ops_impl;
    use crate::os::Os;
    use crate::os::OsArch::X64;
    use crate::package::{Package, SemVerRev, Software};
    use crate::tmp::TmpWorkingDir;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct JetbrainsToolboxInfo {
        version: SemVerRev,
        hash_sha256: String,
    }

    pub fn jetbrains_toolbox_rel_dir() -> PathBuf {
        PathBuf::new()
            .join(".local")
            .join("share")
            .join("JetBrains")
            .join("Toolbox")
    }

    pub fn is_jetbrains_toolbox_installed() -> Result<bool, String> {
        let rel_dir = jetbrains_toolbox_rel_dir();

        env::var("HOME")
            .map(|home| PathBuf::from(&home).join(rel_dir))
            .map_err(|error| error.to_string())?
            .try_exists()
            .map_err(|error| error.to_string())
    }

    pub fn open_jetbrains_toolbox() -> Result<Output, String> {
        let toolbox_bin = env::var("HOME")
            .map(|home| PathBuf::from(&home))
            .map_err(|error| error.to_string())?
            .join(".local")
            .join("share")
            .join("JetBrains")
            .join("Toolbox")
            .join("bin")
            .join("jetbrains-toolbox");

        // TODO change the approach.
        // TODO if the app is opened it probably does nothing
        // TODO if it's closed it waits forever
        exec_cmd(toolbox_bin.to_str().unwrap(), &[])
            .map_err(|error| error.to_string())
    }

    pub struct JetBrainsToolboxImage(DesktopImage);

    impl JetBrainsToolboxImage {
        pub fn new(
            os: Os,
            JetbrainsToolboxInfo { version, hash_sha256 }: JetbrainsToolboxInfo,
        ) -> Self {
            let id = JetBrainsToolbox;
            let pkg_name = id.to_string();
            let fetch_url = match os {
                Linux(X64, _) => format!("https://download.jetbrains.com/toolbox/jetbrains-toolbox-{version}.tar.gz")
            };
            let hash = Hash::new(Sha256, hash_sha256);

            JetBrainsToolboxImage(DesktopImage(
                id,
                Package::new(
                    &pkg_name,
                    os,
                    Software::new("JetBrains s.r.o.", "JetBrains Toolbox", &version.to_string()),
                    Url::parse("https://www.jetbrains.com/toolbox-app").unwrap(),
                    DownloadRequest::new(&fetch_url, Integrity::Hash(hash)).unwrap(),
                ),
            ))
        }
    }

    impl Install for JetBrainsToolboxImage {
        fn install(&self) -> Result<(), String> {
            println!("Installing dependencies (FUSE)...");

            let output = exec_cmd(
                "sudo",
                &["apt-get", "install", "libfuse2"],
            ).map_err(|error| error.to_string())?;

            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

            let tmp = TmpWorkingDir::new()
                .map_err(|error| error.to_string())?;

            let tmp_path = tmp.path();
            let downloader = Downloader::from(self.0.package().fetch, &tmp);
            let tar_file = downloader.path.clone();

            println!("Downloading JetBrains Toolbox installer...");

            downloader
                .download_blocking()
                .map_err(|error| error.to_string())?;

            println!("Extracting JetBrains Toolbox installer...");

            let output = exec_cmd(
                "tar",
                &[
                    "-xvf",
                    tar_file.to_str().unwrap(),
                    "--directory",
                    tmp_path.to_str().unwrap(),
                ],
            ).map_err(|error| error.to_string())?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            let installer_rel_path = stdout
                .lines()
                .last() // The tar only contains one single file (the installer binary)
                .ok_or("Fail to read installer path from output of command tar")?;

            println!("stdout: {}", stdout);
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

            println!("Installing JetBrains Toolbox...");

            let installer_file = tmp_path.join(installer_rel_path);
            let install_cmd = format!("{}", installer_file.to_str().unwrap());
            let output = exec_cmd(&install_cmd, &[])
                .map_err(|error| error.to_string())?;

            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            println!("JetBrains Toolbox installed.");

            Ok(())
        }
    }

    impl Uninstall for JetBrainsToolboxImage {
        fn uninstall(&self) -> Result<(), String> {
            println!("Uninstalling JetBrains Toolbox softly, IDEs will keep installed...");

            let home = env::var("HOME")
                .map(|home| PathBuf::from(&home))
                .map_err(|error| error.to_string())?;

            // Delete autostart file
            let toolbox_autostart_file = home
                .join(".config")
                .join("autostart")
                .join("jetbrains-toolbox.desktop");

            fs::remove_file(toolbox_autostart_file)
                .map_err(|error| error.to_string())?;

            // Delete Toolbox files but ./apps
            let toolbox_dir = home.join(jetbrains_toolbox_rel_dir());
            let dont_delete = toolbox_dir.join("apps");

            let toolbox_entries = fs::read_dir(toolbox_dir)
                .map_err(|error| error.to_string())?
                .filter_map(|res| res.ok())
                .map(|child| child.path())
                .filter(|path| *path != dont_delete);

            for entry in toolbox_entries {
                if entry.is_dir() {
                    fs::remove_dir_all(&entry).map_err(|error| error.to_string())?;
                } else {
                    fs::remove_file(&entry).map_err(|error| error.to_string())?;
                }
            }

            // Delete applications desktop file
            let apps_toolbox_file = home
                .join(".local")
                .join("share")
                .join("applications")
                .join("jetbrains-toolbox.desktop");

            fs::remove_file(apps_toolbox_file)
                .map_err(|error| error.to_string())?;

            println!("JetBrains Toolbox uninstalled.");

            Ok(())
        }
    }

    impl ImageOps for JetBrainsToolboxImage { image_ops_impl!(); }
}

pub mod pycharm {
    use crate::cmd::exec_cmd;
    use crate::download::hashing::Hash;
    use crate::download::hashing::HashAlgorithm::Sha256;
    use crate::download::{DownloadRequest, Downloader, Integrity};
    use crate::image::desktop::jetbrains_toolbox::{is_jetbrains_toolbox_installed, jetbrains_toolbox_rel_dir, open_jetbrains_toolbox};
    use crate::image::desktop::DesktopImage;
    use crate::image::desktop::DesktopImageId::PyCharm;
    use crate::image::Image;
    use crate::image::{ImageOps, Install, Uninstall};
    use crate::os::Os;
    use crate::os::Os::Linux;
    use crate::os::OsArch::X64;
    use crate::package::{Package, Software, YearSemVer};
    use crate::tmp::TmpWorkingDir;
    use crate::{cmd, image_ops_impl};
    use reqwest::Url;
    use serde::{Deserialize, Serialize};
    use std::path::{Path, PathBuf};
    use std::{env, fs};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct PyCharmInfo {
        version: YearSemVer,
        hash_sha256: String,
    }

    pub struct PyCharmImage(DesktopImage);

    impl PyCharmImage {
        pub fn new(
            os: Os,
            PyCharmInfo { version, hash_sha256 }: PyCharmInfo,
        ) -> Self {
            let id = PyCharm;
            let pkg_name = id.to_string();
            let fetch_url = match os {
                Linux(X64, _) => format!("https://download.jetbrains.com/python/pycharm-professional-{version}.tar.gz")
            };
            let hash = Hash::new(Sha256, hash_sha256);

            PyCharmImage(DesktopImage(
                id,
                Package::new(
                    &pkg_name,
                    os,
                    Software::new("JetBrains s.r.o.", "PyCharm", &version.to_string()),
                    Url::parse("https://www.jetbrains.com/pycharm/download").unwrap(),
                    DownloadRequest::new(&fetch_url, Integrity::Hash(hash)).unwrap(),
                ),
            ))
        }
    }

    impl Install for PyCharmImage {
        fn install(&self) -> Result<(), String> {
            let is_toolbox_installed = is_jetbrains_toolbox_installed()?;

            if !is_toolbox_installed {
                return Err("JetBrains Toolbox is required to install JetBrains IDEs but is not installed in your system. Install JetBrains Toolbox first.".to_string());
            }

            println!("Installing PyCharm");

            let tmp = TmpWorkingDir::new()
                .map_err(|error| error.to_string())?;

            let tmp_path = tmp.path();
            let downloader = Downloader::from(self.0.package().fetch, &tmp);
            let tar_file = downloader.path.clone();

            println!("Downloading PyCharm...");

            downloader
                .download_blocking()
                .map_err(|error| error.to_string())?;

            println!("Extracting PyCharm...");

            let home = env::var("HOME")
                .map(|home| PathBuf::from(&home))
                .map_err(|error| error.to_string())?;

            let toolbox_rel_dir = jetbrains_toolbox_rel_dir();
            let apps_dir = home
                .join(toolbox_rel_dir.clone())
                .join("apps");

            let output = exec_cmd(
                "tar",
                &[
                    "-xf",
                    tar_file.to_str().unwrap(),
                    "--directory",
                    tmp_path.to_str().unwrap(),
                ],
            ).map_err(|error| error.to_string())?;

            cmd::print_output(output);

            println!("Moving PyCharm files...");

            let version = self.0.package().software.version;
            let extracted_dir_name = format!("pycharm-{version}");
            let extracted_dir_rel_path = Path::new(&extracted_dir_name);
            let pycharm_tmp_dir = tmp_path.join(extracted_dir_rel_path);
            let pycharm_dir = apps_dir.join("pycharm");

            fs::rename(pycharm_tmp_dir, pycharm_dir.clone())
                .map_err(|error| error.to_string())?;

            println!("Opening JetBrains Toolbox to complete the installation...");

            let output = open_jetbrains_toolbox()?;

            cmd::print_output(output);

            println!("PyCharm installed.");

            Ok(())
        }
    }

    impl Uninstall for PyCharmImage {
        fn uninstall(&self) -> Result<(), String> {
            println!("Uninstalling PyCharm");

            let home = env::var("HOME")
                .map(|home| PathBuf::from(&home))
                .map_err(|error| error.to_string())?;

            let toolbox_dir = home
                .join(".local")
                .join("share")
                .join("JetBrains")
                .join("Toolbox");

            println!("Removing PyCharm files...");

            let pycharm_dir = toolbox_dir
                .join("apps")
                .join("pycharm");

            fs::remove_dir_all(pycharm_dir)
                .map_err(|error| error.to_string())?;

            println!("Opening JetBrains Toolbox to complete the uninstallation...");

            let output = open_jetbrains_toolbox()?;

            cmd::print_output(output);

            println!("PyCharm uninstalled.");

            Ok(())
        }
    }

    impl ImageOps for PyCharmImage { image_ops_impl!(); }
}
