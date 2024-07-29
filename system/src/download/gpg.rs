// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use reqwest::Url;

use crate::cmd::exec_cmd;

pub struct GpgKey {
    url: Url,
    fingerprint: String,
}

impl GpgKey {
    pub fn new(key_url: Url, key_fingerprint: String) -> Self {
        GpgKey { url: key_url, fingerprint: key_fingerprint }
    }

    pub fn install(&self) -> Result<(), String> {
        let curl_cmd = format!("curl --proto '=https' --tlsv1.2 -sSf {} | gpg --import -", self.url);
        let cmd_output = exec_cmd("bash", &["-c", &curl_cmd])
            .map_err(|error| error.to_string())?;

        let stdout = String::from_utf8_lossy(&cmd_output.stdout);

        println!("{}", stdout);

        self.check_key_fingerprint()?;

        println!("GPG key installed");

        Ok(())
    }

    fn check_key_fingerprint(&self) -> Result<(), String> {
        let cmd_output = exec_cmd("gpg", &["--fingerprint"])
            .map_err(|error| error.to_string())?;

        let stdout = String::from_utf8_lossy(&cmd_output.stdout);

        match self.gpg_output_contains_fingerprint(&stdout) {
            true => Ok(()),
            false => Err("Key fingerprint does not exist in GPG".to_string())
        }
    }

    fn gpg_output_contains_fingerprint(&self, output: &str) -> bool {
        let no_whitespace = |c: &char| !c.is_whitespace();

        let normalized_output: String = output
            .chars()
            .filter(no_whitespace)
            .collect();

        let normalized_fingerprint: String = self
            .fingerprint
            .chars()
            .filter(no_whitespace)
            .collect();

        normalized_output.contains(&normalized_fingerprint)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Url;

    use crate::download::gpg::GpgKey;

    #[test]
    pub fn installs_zoom_gpg_key() {
        let url = "https://zoom.us/linux/download/pubkey?version=5-12-6";
        let fingerprint = "59C8 6188 E22A BB19 BD55 4047 7B04 A1B8 DD79 B481";
        let key = GpgKey::new(Url::parse(url).unwrap(), fingerprint.to_string());

        key.install().expect("Fail to install Zoom GPG key");
    }

    #[test]
    pub fn rejects_invalid_fingerprint() {
        let url = "https://zoom.us/linux/download/pubkey?version=5-12-6";

        // Has a 0 instead of the original 5 at the fist character
        let fingerprint = "09C8 6188 E22A BB19 BD55 4047 7B04 A1B8 DD79 B481";
        let key = GpgKey::new(Url::parse(url).unwrap(), fingerprint.to_string());

        key.install().expect_err("Fail to reject wrong GPG fingerprint");
    }
}