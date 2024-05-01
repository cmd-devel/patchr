use log::LevelFilter;

macro_rules! resource_path {
    ($r:literal) => {
        concat!("../../../resources/", $r)
    };
}

pub fn init_logger() {
    let config_str = include_str!(resource_path!("log4rs.yaml"));
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
    // default level
    log::set_max_level(log::LevelFilter::Info);
}

pub fn next_verbose_level(level: LevelFilter) -> LevelFilter {
    match level {
        LevelFilter::Off => LevelFilter::Error,
        LevelFilter::Error => LevelFilter::Warn,
        LevelFilter::Warn => LevelFilter::Info,
        LevelFilter::Info => LevelFilter::Debug,
        LevelFilter::Debug => LevelFilter::Trace,
        LevelFilter::Trace => LevelFilter::Trace,
    }
}
