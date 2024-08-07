// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::fmt;
use std::str::FromStr;

use ServerImageId::{Go, Gradle, Java, Rust, Sdkman};

use crate::image::{Image, ImageId, StrFind, ToImageId};
use crate::impl_image;
use crate::package::Package;

#[derive(Clone, Debug)]
pub enum ServerImageId {
    Rust,
    Go,
    Sdkman,
    Java,
    Gradle,
}

impl Display for ServerImageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Rust => "rust",
            Go => "go",
            Sdkman => "sdkman",
            Java => "java",
            Gradle => "gradle",
        };

        write!(f, "{}", msg)
    }
}

impl StrFind for ServerImageId {
    fn str_find(s: &str) -> Option<Self> {
        match s {
            "rust" => Some(Rust),
            "go" => Some(Go),
            "sdkman" => Some(Sdkman),
            "java" => Some(Java),
            "gradle" => Some(Gradle),
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

    use crate::cmd::exec_cmd;
    use crate::download::{DownloadRequest, Integrity};
    use crate::image::{Image, ImageOps, Install, Uninstall};
    use crate::image::server::ServerImage;
    use crate::image::server::ServerImageId::Rust;
    use crate::image_ops_impl;
    use crate::os::Os;
    use crate::os::Os::Linux;
    use crate::package::{Package, Software};

    pub struct RustImage(ServerImage);

    impl RustImage {
        pub fn new(os: Os) -> Self {
            let id = Rust;
            let pkg_id = id.to_string();
            let fetch_url = match os {
                Linux(_, _) => "https://sh.rustup.rs"
            };
            let version = "latest";

            RustImage(
                ServerImage(
                    id,
                    Package::new(
                        &pkg_id,
                        os,
                        Software::new("Rust Team", "Rust", version),
                        Url::parse("https://www.rust-lang.org/tools/install").unwrap(),
                        DownloadRequest::new(fetch_url, Integrity::None).unwrap(),
                    )))

            // More Rustup doc:
            // https://rust-lang.github.io/rustup/installation/other.html
        }
    }

    impl Install for RustImage {
        fn install(&self) -> Result<(), String> {
            let bash_cmd = format!("curl --proto '=https' --tlsv1.2 -sSf {} | sh -s -- -y", self.0.package().fetch.url());
            let output = exec_cmd("bash", &["-c", &bash_cmd])
                .map_err(|output| output.to_string())?;

            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            Ok(())
        }
    }

    impl Uninstall for RustImage {
        fn uninstall(&self) -> Result<(), String> {
            let output = exec_cmd("rustup", &["self", "uninstall", "-y"])
                .map_err(|output| output.to_string())?;

            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            Ok(())
        }
    }

    impl ImageOps for RustImage { image_ops_impl!(); }
}

pub mod go {
    use std::env;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::path::Path;

    use reqwest::Url;
    use serde::{Deserialize, Serialize};

    use crate::cmd::exec_cmd;
    use crate::download::{Downloader, DownloadRequest, Integrity};
    use crate::image::{Image, ImageOps, Install, Uninstall};
    use crate::image::server::ServerImage;
    use crate::image::server::ServerImageId::Go;
    use crate::image_ops_impl;
    use crate::os::Os;
    use crate::os::Os::Linux;
    use crate::package::{Package, SemVer, Software};
    use crate::tmp::TmpWorkingDir;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GoInfo {
        version: SemVer,
    }

    pub struct GoImage(ServerImage);

    impl GoImage {
        pub fn new(os: Os, GoInfo { version }: GoInfo) -> Self {
            let id = Go;
            let fetch_url = match os {
                Linux(_, _) => format!("https://go.dev/dl/go{}.linux-amd64.tar.gz", version),
            };

            GoImage(
                ServerImage(
                    id.clone(),
                    Package::new(
                        &id.to_string(),
                        os,
                        Software::new("Google, LLC", "Go", &version.to_string()),
                        Url::parse("https://go.dev/doc/install").unwrap(),
                        DownloadRequest::new(&fetch_url, Integrity::None).unwrap(),
                    )))
        }
    }

    impl Install for GoImage {
        fn install(&self) -> Result<(), String> {
            let root_install_dir = Path::new("/usr/local");

            // Do not untar the archive into an existing /usr/local/go tree.
            // This is known to produce broken Go installations. Source: Go Doc.
            remove_go_dir()?;

            let package = self.0.package();
            let tmp = TmpWorkingDir::new()
                .map_err(|error| error.to_string())?;

            let downloader = Downloader::from(package.fetch.clone(), &tmp);
            let installer_file = downloader.path.clone();

            println!("Downloading Go...");

            downloader
                .download_blocking()
                .map_err(|error| error.to_string())?;

            println!("Unpacking Go...");

            let output = exec_cmd(
                "sudo",
                &["tar", "-C", root_install_dir.to_str().unwrap(), "-xzf", installer_file.to_str().unwrap()],
            ).map_err(|error| error.to_string())?;
            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            println!("Updating environment variable...");

            let home = env::var("HOME").unwrap();
            let mut prof = OpenOptions::new()
                .write(true)
                .append(true)
                .open(Path::new(&home).join(".profile"))
                .map_err(|error| error.to_string())?;

            writeln!(prof, "# Golang").map_err(|error| error.to_string())?;
            writeln!(prof, r#"export PATH="$PATH:/usr/local/go/bin""#).map_err(|error| error.to_string())?;
            writeln!(prof, "").map_err(|error| error.to_string())?;

            let output = exec_cmd(
                "bash",
                &["-c", "source ~/.profile && go version"],
            ).map_err(|error| error.to_string())?;
            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            println!("Go installed.");

            Ok(())
        }
    }

    impl Uninstall for GoImage {
        fn uninstall(&self) -> Result<(), String> {
            println!("Removing Go files...");

            remove_go_dir()?;

            println!("Cleaning environment variable...");

            // It deletes the lines from ~/.profile
            // # Golang
            // export PATH="$PATH:/usr/local/go/bin"
            //
            let prof = env::var("HOME")
                .map(|home| Path::new(&home).join(".profile"))
                .map_err(|output| output.to_string())?;

            let clean_profile_pattern = r#"/# Golang/d; /export PATH="\$PATH:\/usr\/local\/go\/bin"/d"#;
            let output = exec_cmd("sed", &["-i", clean_profile_pattern, prof.to_str().unwrap()])
                .map_err(|output| output.to_string())?;

            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            println!("Go uninstalled.");

            Ok(())
        }
    }

    impl ImageOps for GoImage { image_ops_impl!(); }

    fn remove_go_dir() -> Result<(), String> {
        let go_install_dir = "/usr/local/go";
        let output = exec_cmd("sudo", &["rm", "-rf", go_install_dir])
            .map_err(|output| output.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        println!("{}", stdout);

        Ok(())
    }
}

pub mod sdkman {
    use std::{env, fs};
    use std::path::Path;

    use reqwest::Url;

    use crate::cmd::exec_cmd;
    use crate::download::{DownloadRequest, Integrity};
    use crate::image::{Image, ImageOps, Install, Uninstall};
    use crate::image::server::ServerImage;
    use crate::image::server::ServerImageId::Sdkman;
    use crate::image_ops_impl;
    use crate::os::Os;
    use crate::package::{Package, Software};

    pub struct SdkmanImage(ServerImage);

    impl SdkmanImage {
        pub fn new(os: Os) -> Self {
            let id = Sdkman;
            let pkg_id = id.to_string();
            let version = "latest";
            let fetch_url = "https://get.sdkman.io";

            SdkmanImage(
                ServerImage(
                    id,
                    Package::new(
                        &pkg_id.as_str(),
                        os,
                        Software::new("SDKMAN!", "SDKMAN!", version),
                        Url::parse("https://sdkman.io/install").unwrap(),
                        DownloadRequest::new(fetch_url, Integrity::None).unwrap(),
                    ),
                )
            )
        }
    }

    impl Install for SdkmanImage {
        fn install(&self) -> Result<(), String> {
            println!("Fetching SDKMAN!");

            let bash_cmd = format!("curl --proto '=https' --tlsv1.2 -sSf {} | bash", self.0.package().fetch.url());
            let output = exec_cmd("bash", &["-c", &bash_cmd])
                .map_err(|output| output.to_string())?;

            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            // sdk is not a program but a bash function declared in
            // sdkman-init.sh, so that script must be sourced first before
            // calling the command.
            // .bashrc should work as well to load the sdk function into the
            // bash session.
            println!("Initializing SDKMAN!");

            let bash_cmd = "source ~/.sdkman/bin/sdkman-init.sh && sdk version";
            let output = exec_cmd("bash", &["-c", &bash_cmd])
                .map_err(|output| output.to_string())?;

            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            if !output.stderr.is_empty() {
                println!("Source .bashrc (error): {}", String::from_utf8_lossy(&output.stderr));
            }

            println!("SDKMAN! installed.");

            Ok(())
        }
    }

    impl Uninstall for SdkmanImage {
        fn uninstall(&self) -> Result<(), String> {
            let sdkman_dir = env::var("HOME")
                .map(|home| Path::new(&home).join(".sdkman"))
                .map_err(|output| output.to_string())?;

            println!("Removing SDKMAN! files...");

            fs::remove_dir_all(sdkman_dir)
                .map_err(|output| output.to_string())?;

            println!("Removing environment variables...");

            // It deletes the lines from ~/.bashrc
            // #THIS MUST BE AT THE END OF THE FILE FOR SDKMAN TO WORK!!!
            // export SDKMAN_DIR="$HOME/.sdkman"
            // [[ -s "$HOME/.sdkman/bin/sdkman-init.sh" ]] && source "$HOME/.sdkman/bin/sdkman-init.sh"
            //

            let prof = env::var("HOME")
                .map(|home| Path::new(&home).join(".bashrc"))
                .map_err(|output| output.to_string())?;

            let clean_profile_pattern = r#"/#THIS MUST BE AT THE END OF THE FILE FOR SDKMAN TO WORK!!!/d; /export SDKMAN_DIR="\$HOME\/.sdkman"/d; /\[\[ -s "\$HOME\/.sdkman\/bin\/sdkman-init.sh" \]\] && source "\$HOME\/.sdkman\/bin\/sdkman-init.sh"/d"#;
            let output = exec_cmd("sed", &["-i", clean_profile_pattern, prof.to_str().unwrap()])
                .map_err(|output| output.to_string())?;

            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            println!("SDKMAN! uninstalled.");

            Ok(())
        }
    }

    impl ImageOps for SdkmanImage { image_ops_impl!(); }
}

pub mod java {
    use reqwest::Url;
    use serde::{Deserialize, Serialize};

    use crate::cmd::exec_cmd;
    use crate::image::{ImageOps, Install, Uninstall};
    use crate::image::Image;
    use crate::image::server::ServerImage;
    use crate::image::server::ServerImageId::Java;
    use crate::image_ops_impl;
    use crate::os::Os;
    use crate::package::{Package, SemVerVendor, Software};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct JavaInfo {
        version: SemVerVendor,
    }

    pub struct JavaImage(ServerImage);

    impl JavaImage {
        pub fn new(os: Os, JavaInfo { version }: JavaInfo) -> Self {
            let id = Java;
            let pkg_name = id.to_string();

            JavaImage(ServerImage(
                id,
                Package::new_managed(
                    &pkg_name,
                    os,
                    Software::new("", "JDK (Java Development Kit)", &version.to_string()),
                    Url::parse("https://sdkman.io/jdks").unwrap(),
                ),
            ))
        }
    }

    impl Install for JavaImage {
        fn install(&self) -> Result<(), String> {
            println!("Installing Java via SDKMAN!");

            let sdk_cmd = format!("sdk install java {}", self.0.package().software.version);
            let bash_cmd = format!("source ~/.sdkman/bin/sdkman-init.sh && {}", sdk_cmd);
            let output = exec_cmd("bash", &["-c", &bash_cmd])
                .map_err(|error| error.to_string())?;

            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            println!("Java installed");

            Ok(())
        }
    }

    impl Uninstall for JavaImage {
        fn uninstall(&self) -> Result<(), String> {
            println!("Uninstalling Java via SDKMAN!");

            let sdk_cmd = format!("sdk uninstall java {} --force", self.0.package().software.version);
            let bash_cmd = format!("source ~/.sdkman/bin/sdkman-init.sh && {}", sdk_cmd);
            let output = exec_cmd("bash", &["-c", &bash_cmd])
                .map_err(|error| error.to_string())?;

            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            println!("Java uninstalled");

            Ok(())
        }
    }

    impl ImageOps for JavaImage { image_ops_impl!(); }
}

pub mod gradle {
    use reqwest::Url;
    use serde::{Deserialize, Serialize};

    use crate::cmd::exec_cmd;
    use crate::image::{ImageOps, Install, Uninstall};
    use crate::image::Image;
    use crate::image::server::ServerImage;
    use crate::image::server::ServerImageId::Gradle;
    use crate::image_ops_impl;
    use crate::os::Os;
    use crate::package::{Package, SemVer, Software};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GradleInfo {
        version: SemVer,
    }

    pub struct GradleImage(ServerImage, SemVer);

    impl GradleImage {
        pub fn new(os: Os, GradleInfo { version }: GradleInfo) -> Self {
            let id = Gradle;
            let pkg_name = id.to_string();

            GradleImage(
                ServerImage(
                    id,
                    Package::new_managed(
                        &pkg_name,
                        os,
                        Software::new("Gradle, Inc", "Gradle", &version.to_string()),
                        Url::parse("https://sdkman.io/sdks").unwrap(),
                    ),
                ),
                version,
            )
        }

        fn get_normalized_version(&self) -> String {
            let SemVer(major, minor, patch) = self.1;

            if patch == 0 {
                format!("{major}.{minor}")
            } else {
                self.1.to_string()
            }
        }
    }

    impl Install for GradleImage {
        fn install(&self) -> Result<(), String> {
            println!("Installing Gradle via SDKMAN!");

            let version = self.get_normalized_version();
            let sdk_cmd = format!("sdk install gradle {version}");
            let bash_cmd = format!("source ~/.sdkman/bin/sdkman-init.sh && {}", sdk_cmd);
            let output = exec_cmd("bash", &["-c", &bash_cmd])
                .map_err(|error| error.to_string())?;

            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            println!("Gradle installed");

            Ok(())
        }
    }

    impl Uninstall for GradleImage {
        fn uninstall(&self) -> Result<(), String> {
            println!("Uninstalling Gradle via SDKMAN!");

            let version = self.get_normalized_version();
            let sdk_cmd = format!("sdk uninstall gradle {version} --force");
            let bash_cmd = format!("source ~/.sdkman/bin/sdkman-init.sh && {}", sdk_cmd);
            let output = exec_cmd("bash", &["-c", &bash_cmd])
                .map_err(|error| error.to_string())?;

            let stdout = String::from_utf8_lossy(&output.stdout);

            println!("{}", stdout);

            println!("Gradle uninstalled");

            Ok(())
        }
    }

    impl ImageOps for GradleImage { image_ops_impl!(); }
}
