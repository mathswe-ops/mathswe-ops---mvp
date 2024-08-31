// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::io;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

pub struct TmpWorkingDir {
    dir: TempDir,
}

impl TmpWorkingDir {
    pub fn new() -> io::Result<Self> {
        let temp_dir = TempDir::with_prefix("mathswe-ops_")?;

        Ok(TmpWorkingDir { dir: temp_dir })
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    pub fn join(&self, path: &Path) -> PathBuf {
        self.path().join(path)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};

    use super::*;

    #[test]
    fn creates_temp_directory() {
        let tmp_working_dir = TmpWorkingDir::new()
            .expect("Failed to create temporary directory");

        assert!(tmp_working_dir.dir.path().exists());

        // Check if the directory is empty
        let entries: Vec<_> = fs::read_dir(tmp_working_dir.dir.path())
            .expect("Failed to read directory")
            .collect();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_join_path() {
        let tmp_working_dir = TmpWorkingDir::new()
            .expect("Failed to create temporary directory");

        let joined_path = tmp_working_dir.join(Path::new("test.txt"));
        assert_eq!(
            joined_path,
            tmp_working_dir.dir.path().join("test.txt")
        );
    }

    #[test]
    fn creates_temp_file() -> io::Result<()> {
        let tmp_working_dir = TmpWorkingDir::new()
            .expect("Failed to create temporary directory");
        let child_path = Path::new("temp_file.txt");
        let file_path = tmp_working_dir.join(child_path);
        let mut temp_file = File::create(&file_path)?;

        temp_file.write_all("Temporary file!".as_ref())?;
        assert!(file_path.exists());

        let mut buffer = String::new();
        let mut read_file = File::open(&file_path)?;
        read_file.read_to_string(&mut buffer)?;
        assert_eq!("Temporary file!", buffer);

        Ok(())
    }
}
