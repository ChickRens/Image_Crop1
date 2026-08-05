use crate::{domain::value_object::session_id::error::SessionIdError, parent_error};

parent_error!(
    pub enum SessionError {
        SessionId(SessionIdError),
    }
);