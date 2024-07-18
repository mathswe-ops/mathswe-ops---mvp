// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;

use sha2::{Digest, Sha256};

pub(crate) fn calculate_sha256(file_path: &Path) -> io::Result<String> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();

    Ok(format!("{:x}", hash))
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::path::Path;

    use crate::download::hashing::calculate_sha256;

    #[test]
    fn checks_sample_file_sha256() -> io::Result<()> {
        let checksum = "0ecfebe350c45dbded8cfb32d3af0b910bde66fc2aafbafabdaaeef6cae48a59";
        let test_file_path = Path::new("resources")
            .join("test")
            .join("download")
            .join("test_file.txt");
        let computed_hash = calculate_sha256(&test_file_path)?;

        assert_eq!(checksum, computed_hash);

        Ok(())
    }
}
