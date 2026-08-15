use crate::common;
use crate::{colorscheme::args, config::Directory};

pub fn handle(args: &args::Remove) {
    common::handle_remove(Directory::Colorschemes, &args.name, "colorscheme");
}
