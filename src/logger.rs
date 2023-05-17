use std::io::Write;

use env_logger::Builder;
use log::{Level, LevelFilter};

pub fn init() {
    init_logger();

    if cfg!(not(debug_assertions)) {
        set_panic_hook();
    }
}

fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|buf, record| {
        let level_style = buf.default_level_style(record.level());

        let level = match record.level() {
            Level::Info => "[*]",
            Level::Warn => "warning:",
            Level::Error => "error:",
            _ => "[?]",
        };

        writeln!(buf, "{} {}", level_style.value(level), record.args())
    });

    builder.filter_level(LevelFilter::Info);
    builder.parse_default_env();

    builder.init();
}

fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        info.payload()
            .downcast_ref::<&str>()
            .map_or_else(|| log::error!("{}", info), |s| log::error!("{}", s));
    }));
}
