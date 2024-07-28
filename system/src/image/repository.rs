// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use ServerImageId::Rust;
use crate::image::{ImageId, ImageInfoError, ImageInfoLoader, ImageLoader, ImageOps, LoadImage, StrFind, ToImageId};
use crate::image::desktop::DesktopImageId;
use crate::image::desktop::DesktopImageId::Zoom;
use crate::image::desktop::zoom::ZoomImage;
use crate::image::server::rust::RustImage;
use crate::image::server::ServerImageId;
use crate::package::Os;

struct RepositoryImageLoader<T> where T: Display + ToImageId {
    id: T,
}

impl Display for RepositoryImageLoader<DesktopImageId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("Desktop Image ID: {}", self.id))
    }
}

impl ToImageId for RepositoryImageLoader<DesktopImageId> {
    fn to_image_id(&self) -> ImageId {
        self.id.to_image_id()
    }
}

impl LoadImage for RepositoryImageLoader<DesktopImageId> {
    fn load_image(&self, os: Os) -> Result<Option<Box<dyn ImageOps>>, ImageInfoError> {
        let info = ImageInfoLoader { root: PathBuf::from("image"), dir: PathBuf::from("") };
        let image = match self.id {
            Zoom => ZoomImage::load_with(os, info)?,
        };

        Ok(image)
    }
}

impl ImageLoader for RepositoryImageLoader<DesktopImageId> {}

impl Display for RepositoryImageLoader<ServerImageId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("Server Image ID: {}", self.id))
    }
}

impl ToImageId for RepositoryImageLoader<ServerImageId> {
    fn to_image_id(&self) -> ImageId {
        self.id.to_image_id()
    }
}

impl LoadImage for RepositoryImageLoader<ServerImageId> {
    fn load_image(&self, os: Os) -> Result<Option<Box<dyn ImageOps>>, ImageInfoError> {
        let image = match self.id {
            Rust => RustImage::from(os)
        };

        Ok(image)
    }
}

impl ImageLoader for RepositoryImageLoader<ServerImageId> {}

pub struct Repository;

impl Repository {
    pub fn image_loader_from(s: &str) -> Result<Box<dyn ImageLoader>, String> {
        if let Some(id) = DesktopImageId::str_find(s) {
            Ok(Self::box_it(id))
        } else if let Some(id) = ServerImageId::str_find(s) {
            Ok(Self::box_it(id))
        } else {
            Err(format!("String ID {} not found in the image repository", s))
        }
    }

    fn box_it<T>(id: T) -> Box<dyn ImageLoader>
    where
        T: Display + ToImageId + 'static,
        RepositoryImageLoader<T>: ImageLoader,
    {
        Box::new(RepositoryImageLoader { id })
    }
}
