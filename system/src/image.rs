// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use serde::de::DeserializeOwned;
use ImageInfoError::{IoError, SerdeError};
use crate::image::images::ImageId;

pub(crate) mod images;

pub trait Install {
    fn install(&self) -> Result<(), String>;
}

pub trait Uninstall {
    fn uninstall(&self) -> Result<(), String>;
}

#[derive(Debug)]
pub enum ImageInfoError {
    IoError(String),
    SerdeError(String),
}

impl Display for ImageInfoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            IoError(msg) => format!("IO Error: {}", msg),
            SerdeError(msg) => format!("Serialization/Deserialization Error: {}", msg),
        };

        write!(f, "{}", msg)
    }
}

pub struct ImageInfoLoader {
    root: PathBuf,
    dir: PathBuf,
}

impl ImageInfoLoader {
    pub fn path(&self, id: ImageId) -> PathBuf {
        self.root.join(self.dir.clone()).join(format!("{}.json", id))
    }

    pub fn load<T: DeserializeOwned>(&self, id: ImageId) -> Result<T, ImageInfoError> {
        let info_path = self.path(id);
        let file = File::open(info_path).map_err(|error| IoError(error.to_string()))?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader)
            .map_err(|error| SerdeError(error.to_string()))
    }
}
