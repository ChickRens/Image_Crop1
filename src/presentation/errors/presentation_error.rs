use crate::{common::traits::ErrorType::{Internal, InvalidInput}, leaf_detail_error};

leaf_detail_error!(
    pub enum PresentationError {
        Multipart => ("MULTIPART_ERROR", Internal),
        UnknownName => ("UNKNOWN_TAG_NAME", InvalidInput),
        NoName => ("NO_TAG_NAME_ERROR", InvalidInput),
    }
);