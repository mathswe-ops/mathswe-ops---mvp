// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::path::Path;

use reqwest::Url;

use crate::cmd::exec_cmd;

#[derive(PartialEq, Clone, Debug)]
pub struct GpgKey {
    url: Url,
    fingerprint: String,
}

impl GpgKey {
    pub fn new(key_url: Url, key_fingerprint: String) -> Self {
        GpgKey { url: key_url, fingerprint: key_fingerprint }
    }

    pub fn verify(&self, file_path: &Path) -> Result<bool, String> {
        let cmd_output = exec_cmd("gpg", &["--verify", file_path.to_str().unwrap()])
            .map_err(|error| error.to_string())?;

        // Use stderr for informational messages
        let stderr = String::from_utf8_lossy(&cmd_output.stderr);

        let out_contains_all = |search: &[&str]| search
            .iter()
            .all(|value| stderr.contains(value));

        let out_contains_required_strings = out_contains_all(&[
            "gpg: Signature made",
            "gpg: Good signature from",
            "Primary key fingerprint:",
        ]);

        let out_has_no_signature_error = !stderr.contains("gpg: verify signatures failed");

        let out_has_fingerprint = self.gpg_output_contains_fingerprint(&stderr);

        let correct
            = out_contains_required_strings
            && out_has_no_signature_error
            && out_has_fingerprint;

        Ok(correct)
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
