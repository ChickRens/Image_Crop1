use crate::{
    application::{
        interface::preview_storage::error::PreviewStorageError, types::preview_image::PreviewImage,
    },
    domain::value_object::image_id::image_id::ImageId,
};

pub trait PreviewStorage {
    fn save(&self, image: PreviewImage);
    fn get(&self, image_id: ImageId) -> Result<PreviewImage, PreviewStorageError>;
}
