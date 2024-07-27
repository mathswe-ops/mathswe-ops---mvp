// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::de::DeserializeOwned;

use ImageInfoError::{IoError, SerdeError};

use crate::package::{Os, Package};

pub(crate) mod repository;
mod desktop;

#[derive(PartialEq, Clone)]
pub struct ImageId(String);

impl Display for ImageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait ToImageId where Self: Display {
    fn to_image_id(&self) -> ImageId;
}

pub trait StrFind {
    fn str_find(s: &str) -> Option<Self> where Self: Sized;
}

pub trait Image: Display {
    fn id(&self) -> ImageId;

    fn package(&self) -> Package;
}

pub trait Install {
    fn install(&self) -> Result<(), String>;
}

pub trait Uninstall {
    fn uninstall(&self) -> Result<(), String>;
}

pub trait ImageOps: Install + Uninstall {
    fn image(&self) -> Box<dyn Image>;
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
    pub root: PathBuf,
    pub dir: PathBuf,
}

impl ImageInfoLoader {
    pub fn path<T: ToImageId>(&self, id: T) -> PathBuf {
        self.root.join(self.dir.clone()).join(format!("{}.json", id))
    }

    pub fn load<D: DeserializeOwned, T: ToImageId>(&self, id: T) -> Result<D, ImageInfoError> {
        let info_path = self.path(id);
        let file = File::open(info_path).map_err(|error| IoError(error.to_string()))?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader)
            .map_err(|error| SerdeError(error.to_string()))
    }
}

pub trait LoadImage where Self: Display {
    fn load_image(&self, os: Os) -> Result<Option<Box<dyn ImageOps>>, ImageInfoError>;
}

pub trait ImageLoader: Display + ToImageId + LoadImage {}
