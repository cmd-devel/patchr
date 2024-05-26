#[cfg(not(windows))]
pub static LINE_SEP: &str = "\n";

#[cfg(windows)]
pub static LINE_SEP: &str = "\r\n";

pub static DEFAULT_DATE_TIME_FORMAT : &str = "%Y-%m-%d %H:%M:%S";