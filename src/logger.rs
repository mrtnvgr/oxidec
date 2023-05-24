use env_logger::Builder;
use log::{Level, LevelFilter};
use std::io::Write;

pub fn init() {
    init_logger();
    cli_panics::set_panic_hook();
}

fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|buf, record| {
        let level_style = buf.default_level_style(record.level());

        let level = match record.level() {
            Level::Info => "*",
            Level::Warn => "!",
            Level::Error => "x",
            _ => "?",
        };

        writeln!(buf, "{} {}", level_style.value(level), record.args())
    });

    builder.filter_level(LevelFilter::Info);
    builder.parse_default_env();

    builder.init();
}
