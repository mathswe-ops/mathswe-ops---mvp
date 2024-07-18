// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::path::PathBuf;

    #[test]
    fn reads_sample_file() -> io::Result<()> {
        let path = PathBuf::from("resources")
            .join("test")
            .join("download")
            .join("test_file.txt");
        let mut file = File::open(path.clone())?;
        let mut buf = String::new();

        file
            .read_to_string(&mut buf)
            .expect(format!("Failed to read resource file {:?}", path).as_str());
        assert!(buf.contains("Lorem ipsum"));

        Ok(())
    }
}
