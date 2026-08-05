use crate::leaf_error;

leaf_error!(
    pub enum ImageSizeError {
        LongHeight => "LONG_HEIGHT",
        LongWidth => "LONG_WIDTH",
        ShortHeight => "SHORT_HEIGHT",
        ShortWidth => "SHORT_WIDTH",
    }
);