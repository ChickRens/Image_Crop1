use crate::application::types::storage_path::StoragePath;

pub struct SegmentOutput{
    path: StoragePath
}

impl SegmentOutput {
    pub fn into_parts(self) -> StoragePath {
        self.path
    }
}