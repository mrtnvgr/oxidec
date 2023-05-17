pub mod args;
mod commands;
mod reloaders;
mod schema;
mod templates;

use args::Action;

pub fn handle(args: &Action) {
    match args {
        Action::Set(args) => commands::set::handle(args),
        Action::Remove(args) => commands::remove::handle(args),
        Action::Import(args) => commands::import::handle(args),
        Action::List(args) => commands::list::handle(args),
        Action::Reload => reloaders::run(),
    }
}
