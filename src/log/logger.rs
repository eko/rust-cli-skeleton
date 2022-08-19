use log::{LevelFilter};
use simplelog::{SimpleLogger, ConfigBuilder};

pub fn init(log_level: LevelFilter) {
    let log_config = ConfigBuilder::new()
        .set_write_log_enable_colors(true)
        .build();

    let _ = SimpleLogger::init(log_level, log_config);
}