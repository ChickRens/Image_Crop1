use crate::{application::{interface::segmenter_input_image_storage::error::SegmenterInputImageStorageError, types::segmenter_input_image::SegmenterInputImage}, domain::value_object::image_id::image_id::ImageId};

pub trait SegmenterInputImageStorage {
    fn save(&self, image: SegmenterInputImage);
    fn get(&self, image_id: ImageId) -> Result<SegmenterInputImage, SegmenterInputImageStorageError>;
}