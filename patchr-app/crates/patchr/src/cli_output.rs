#[macro_export]
macro_rules! cli_print {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}

#[macro_export]
macro_rules! cli_print_error {
    ($($arg:tt)*) => {
        eprintln!("[Error] {}", format!($($arg)*))
    };
}
