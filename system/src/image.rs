// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::de::DeserializeOwned;

use ImageInfoError::{IoError, SerdeError};

use crate::os::Os;
use crate::package::Package;

pub(crate) mod repository;
mod desktop;
mod server;

#[derive(PartialEq, Clone, Debug)]
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

#[macro_export]
macro_rules! impl_image {
    ($struct_name:ident) => {
        impl Display for $struct_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "Image: {:?}, Package: {:?}", self.id(), self.package())
            }
        }

        impl Image for $struct_name {
            fn id(&self) -> ImageId {
                self.0.to_image_id()
            }

            fn package(&self) -> Package {
                self.1.clone()
            }
        }
    };
}

pub trait Install {
    fn install(&self) -> Result<(), String>;
}

pub trait Uninstall {
    fn uninstall(&self) -> Result<(), String>;
}

pub trait ImageOps: Install + Uninstall {
    fn image(&self) -> Box<dyn Image>;

    fn reinstall(&self) -> Result<(), String> {
        self.uninstall()?;
        self.install()?;
        Ok(())
    }
}

#[macro_export]
macro_rules! image_ops_impl {
    () => {
        fn image(&self) -> Box<dyn Image> {
            Box::new(self.0.clone())
        }
    };
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
    id: ImageId,
    root: PathBuf,
    dir: PathBuf,
}

impl ImageInfoLoader {
    pub fn from<T: Clone + ToImageId>(id: &T, root: PathBuf, dir: PathBuf) -> Self {
        ImageInfoLoader { id: id.clone().to_image_id(), root, dir }
    }

    pub fn path(&self) -> PathBuf {
        self.root.join(self.dir.clone()).join(format!("{}.json", self.id))
    }

    pub fn load<D: DeserializeOwned>(&self) -> Result<D, ImageInfoError> {
        let info_path = self.path();
        let file = File::open(info_path.clone())
            .map_err(|error| IoError(
                format!("Fail to read image info at {:?}.\nCause: {}", info_path, error.to_string())
            ))?;

        let reader = BufReader::new(file);

        serde_json::from_reader(reader)
            .map_err(|error| SerdeError(error.to_string()))
    }
}

pub struct ImageLoadContext {
    os: Os,
    info_loader: ImageInfoLoader,
}

impl ImageLoadContext {
    pub fn new(os: &Os, info_loader: ImageInfoLoader) -> Self {
        ImageLoadContext { os: os.clone(), info_loader }
    }

    fn image_from<D: DeserializeOwned, T: ImageOps + 'static>(
        os: Os,
        info: D,
        cons: fn(Os, D) -> T
    ) -> Option<Box<dyn ImageOps>> {
        Some(Box::new(cons(os, info)))
    }

    pub fn load<D: DeserializeOwned, T: ImageOps + 'static>(
        &self,
        cons: fn(Os, D) -> T,
    ) -> Result<Option<Box<dyn ImageOps>>, ImageInfoError> {
        let info = self.info_loader.load()?;
        let image = Self::image_from(self.os.clone(), info, cons);

        Ok(image)
    }
}

pub trait LoadImage where Self: Display {
    fn load_image(&self, os: Os) -> Result<Option<Box<dyn ImageOps>>, ImageInfoError>;
}

pub trait ImageLoader: Display + ToImageId + LoadImage {}
