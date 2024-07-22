// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::Display;

pub(crate) mod packages;

#[derive(PartialEq)]
pub struct SemVer(u8, u8, u8);

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

#[derive(PartialEq)]
pub struct SemVerRev(u8, u8, u8, u16);

impl Display for SemVerRev {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

pub trait Install<E> {
    fn install(&self) -> Result<(), E>;
}

pub trait Uninstall<E> {
    fn uninstall(&self) -> Result<(), E>;
}

#[cfg(test)]
mod tests {
    use crate::package::{SemVer, SemVerRev};

    #[test]
    fn semver_to_string() {
        let ver = SemVer(2, 10, 6);

        assert_eq!("2.10.6", ver.to_string())
    }

    #[test]
    fn semver_rev_to_string() {
        let ver = SemVerRev(2, 10, 6, 465);

        assert_eq!("2.10.6.465", ver.to_string())
    }
}
