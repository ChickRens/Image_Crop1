use crate::leaf_error;

leaf_error!(
    pub enum SessionIdError {
        InvalidSessionId => "INVALID_SESSION_ID",
    }
);