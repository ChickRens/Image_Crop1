use crate::{domain::entity::{image::error::ImageError, session::error::SessionError}, parent_error};

parent_error!(
    pub enum DomainErrors {
        ImageError(ImageError),
        SessionError(SessionError),
    }
);