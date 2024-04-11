pub mod args;
pub mod blocks;
pub mod commands;
pub mod reloaders;
pub mod schema;
pub mod templates;

use args::Action;

pub fn handle(args: Action) {
    match args {
        Action::Set(args) => commands::set::handle(args),
        Action::Remove(args) => commands::remove::handle(args),
        Action::Import(args) => commands::import::handle(args),
        Action::List(args) => commands::list::handle(args),
        Action::Status(args) => commands::status::handle(args),
        Action::Generate(args) => commands::generate::handle(args),
        Action::Reload => reloaders::run(false),
    }
}
