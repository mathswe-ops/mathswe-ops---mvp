// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub enum Software {}

impl FromStr for Software {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s { _ => Err(format!("String {} does not map to enum Software", s)) }
    }
}

#[derive(Debug)]
pub struct Package {
    name: String,
    software: Software,
}

impl Package {
    pub fn from(name: String) -> Result<Package, String> {
        Software::from_str(&name)
            .map(|software| Package { name, software })
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("Package name: {}, Software: {:?}", self.name, self.software))
    }
}
