pub trait Code {
    fn code(&self) -> &str;
}

pub trait Cause {
    fn cause(&self) -> Option<&str>;
}
