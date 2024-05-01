macro_rules! declare_env {
    ($name:ident) => {
        pub const $name: &str = stringify!($name);
    };
}

// Set max debug level
declare_env!(PATCHR_DBG);
