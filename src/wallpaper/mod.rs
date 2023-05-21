pub mod args;
mod commands;
mod set;

use args::Action;

pub fn handle(args: &Action) {
    match args {
        Action::Add(args) => commands::add::handle(args),
        Action::Set(args) => commands::set::handle(args),
        Action::Remove(args) => commands::remove::handle(args),
        Action::List(args) => commands::list::handle(args),
        Action::Status(args) => commands::status::handle(args),
    }
}
