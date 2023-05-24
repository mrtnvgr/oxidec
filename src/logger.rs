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

fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let message = match (
            info.payload().downcast_ref::<&str>(),
            info.payload().downcast_ref::<String>(),
        ) {
            (Some(s), _) => Some((*s).to_owned()),
            (_, Some(s)) => Some(s.to_string()),
            (None, None) => None,
        };

        message.map_or_else(
            || log::error!("{}", info),
            |message| log::error!("{}", message),
        );
    }));
}
