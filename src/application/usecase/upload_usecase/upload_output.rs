use crate::application::types::storage_path::StoragePath;

pub struct UploadOutput{
    path: StoragePath
}

impl UploadOutput {
    pub fn into_parts(self) -> StoragePath {
        self.path
    }
}