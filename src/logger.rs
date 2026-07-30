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
        let level = record.level();
        let text = record.args();

        let style = buf.default_level_style(level);

        if matches!(level, Level::Warn | Level::Error) {
            writeln!(buf, "{style}{text}{style:#}")
        } else {
            writeln!(buf, "{text}")
        }
    });

    builder.filter_level(LevelFilter::Info);
    builder.parse_default_env();

    builder.init();
}
