use crate::common;
use crate::{config::Directory, theme::args};

pub fn handle(args: &args::Remove) {
    common::handle_remove(Directory::Themes, &args.name, "theme");
}
