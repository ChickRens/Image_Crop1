use crate::leaf_error;

leaf_error!(
    pub enum EditingSessionRepositoryError {
        EditingSessionNotFound => "EDITING_SESSION_NOT_FOUND",
    }
);
