use crate::application::types::storage_path::StoragePath;
use crate::ImageId;

pub struct UploadOutput{
    image_id: ImageId
}

impl UploadOutput {
    pub fn into_parts(self) -> StoragePath {
        self.path
    }
}