pub mod args;
mod commands;
mod schema;

use args::Action;

pub fn handle(args: Action) {
    match args {
        Action::Set(args) => commands::set::handle(&args),
        Action::Create(args) => commands::create::handle(&args),
        Action::Remove(args) => commands::remove::handle(&args),
        Action::List(args) => commands::list::handle(&args),
    }
}
