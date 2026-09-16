use std::ops::DerefMut;

use crate::{
    application::{
        interface::preview_storage::error::PreviewStorageError, types::preview_image::PreviewImage,
    },
    domain::value_object::image_id::image_id::ImageId,
};

pub trait PreviewStorage {
    type Guard<'a>: DerefMut<Target = PreviewImage>
    where
        Self: 'a;

    fn save_as_original(&self, original: PreviewImage);
    fn save_as_segmented(&self, segmented: PreviewImage);
    fn get<'a>(&'a self, image_id: ImageId) -> Result<Self::Guard<'a>, PreviewStorageError>;
}