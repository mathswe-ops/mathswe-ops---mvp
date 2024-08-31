// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::de::DeserializeOwned;

use crate::image::ImageOperationError::{InfoError, OperationNotImplemented};
use crate::os::Os;
use crate::package::Package;
use ImageInfoError::{IoError, SerdeError};

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

pub trait ImageOperation {
    fn image_id(&self) -> ImageId;
}

pub trait Config: ImageOperation {
    fn config(&self) -> Result<(), String>;
}

pub struct ImageConfig<I, C>(I, C) where I: ImageOps, C: DeserializeOwned;

pub trait ToImageConfig<C> where Self: ImageOps + Sized, C: DeserializeOwned {
    fn to_image_config(&self, config: C) -> ImageConfig<Self, C>;
}

impl<I, C> ImageOperation for ImageConfig<I, C>
where
    I: ImageOps,
    C: DeserializeOwned,
{
    fn image_id(&self) -> ImageId {
        self.0.image().id()
    }
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

#[derive(Debug)]
pub enum ImageOperationError {
    OperationNotImplemented(ImageId, String),
    InfoError(ImageInfoError),
}

impl ImageOperationError {
    fn from_image_info_error(error: ImageInfoError) -> Self {
        InfoError(error)
    }
}

impl Display for ImageOperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            OperationNotImplemented(id, op) =>
                format!("Operation {op} not implemented for image {id}"),

            InfoError(error) => error.to_string(),
        };

        write!(f, "{}", msg)
    }
}

pub enum InfoFileType {
    Image,
    Config,
}

impl Display for InfoFileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            InfoFileType::Image => "image",
            InfoFileType::Config => "config",
        };

        write!(f, "{}", msg)
    }
}

pub struct ImageInfoLoader {
    id: ImageId,
    root: PathBuf,
    dir: PathBuf,
    file_type: InfoFileType,
}

impl ImageInfoLoader {
    pub fn from<T: Clone + ToImageId>(
        concrete_id: &T,
        root: PathBuf,
        dir: PathBuf,
    ) -> Self {
        let id = concrete_id.clone().to_image_id();
        let file_type = InfoFileType::Image;

        ImageInfoLoader { id, root, dir, file_type }
    }

    pub fn of(&self, file_type: InfoFileType) -> Self {
        Self {
            id: self.id.clone(),
            root: self.root.clone(),
            dir: self.dir.clone(),
            file_type,
        }
    }

    pub fn path(&self) -> PathBuf {
        let dir = self.root.join(self.dir.clone());

        let filename = match self.file_type {
            InfoFileType::Image => format!("{}.json", self.id),
            _ => format!("{}.{}.json", self.id, self.file_type),
        };

        dir.join(filename)
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

    pub fn basic_image_from<T: ImageOps + 'static>(
        os: Os,
        cons: fn(Os) -> T,
    ) -> Box<dyn ImageOps> {
        Box::new(cons(os))
    }

    fn image_from<D: DeserializeOwned, T: ImageOps + 'static>(
        os: Os,
        info: D,
        cons: impl Fn(Os, D) -> T,
    ) -> Box<dyn ImageOps> {
        Box::new(cons(os, info))
    }

    pub fn load<D: DeserializeOwned, T: ImageOps + 'static>(
        &self,
        cons: impl Fn(Os, D) -> T,
    ) -> Result<Box<dyn ImageOps>, ImageInfoError> {
        let info = self.info_loader.load()?;
        let image = Self::image_from(self.os.clone(), info, cons);

        Ok(image)
    }

    pub fn load_concrete<D: DeserializeOwned, T: ImageOps + 'static>(
        &self,
        cons: impl Fn(Os, D) -> T,
    ) -> Result<T, ImageOperationError> {
        let info = self
            .info_loader
            .load()
            .map_err(ImageOperationError::from_image_info_error)?;

        let image = cons(self.os.clone(), info);

        Ok(image)
    }

    pub fn load_config<D: DeserializeOwned>(
        &self,
    ) -> Result<D, ImageInfoError> {
        let config = self
            .info_loader
            .of(InfoFileType::Config)
            .load()?;

        Ok(config)
    }

    pub fn load_to_image_config<D, T>(
        &self,
        image: T,
    ) -> Result<Box<dyn Config>, ImageOperationError>
    where
        D: DeserializeOwned + 'static,
        T: ImageOps + ToImageConfig<D> + 'static,
        ImageConfig<T, D>: Config,
    {
        let config = self
            .load_config()
            .map(|config| image.to_image_config(config))
            .map_err(ImageOperationError::from_image_info_error)?;

        Ok(Box::new(config))
    }
}

pub trait LoadImage where Self: Display {
    fn load_image(&self, os: Os) -> Result<Box<dyn ImageOps>, ImageInfoError>;

    fn load_config(&self, os: Os) -> Result<Box<dyn Config>, ImageOperationError>;
}

pub trait ImageLoader: Display + ToImageId + LoadImage {}

#[cfg(test)]
mod tests {
    use crate::image::{ImageId, ImageInfoLoader, InfoFileType};
    use std::path::PathBuf;

    #[test]
    fn image_info_path() {
        let info = ImageInfoLoader {
            id: ImageId("image_name".to_string()),
            root: PathBuf::from("image"),
            dir: PathBuf::from(""),
            file_type: InfoFileType::Image,
        };

        assert_eq!(
            PathBuf::from("image/image_name.json"),
            info.path(),
        );

        let config = info.of(InfoFileType::Config);

        assert_eq!(
            PathBuf::from("image/image_name.config.json"),
            config.path(),
        );
    }
}
