use crate::leaf_error;

leaf_error!(
    pub enum EditingSessionError {
        UndoFailed => "UNDO_FAILED",
        RedoFailed => "REDO_FAILED",
        HistoryCorrupted => "HISTORY_CORRUPTED",
    }
);
