#[cfg(not(windows))]
pub static LINE_SEP: &str = "\n";

#[cfg(windows)]
pub static LINE_SEP: &str = "\r\n";
