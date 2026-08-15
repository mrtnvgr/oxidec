use crate::common;
use crate::{colorscheme::args, config::Directory};

pub fn handle(args: &args::List) {
    common::handle_list(Directory::Colorschemes.list_stems(), args.json);
}
