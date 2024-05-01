#[derive(Clone, Copy)]
pub enum ErrorCode {
    CannotReadUserData = 1,
    CannotWriteUserData = 2,
    CommandError = 3,
    ParsingError = 4,
}

impl ErrorCode {
    pub fn code(&self) -> i32 {
        *self as i32
    }
}
