use crate::common;
use crate::{config::Directory, theme::args};

pub fn handle(args: &args::List) {
    common::handle_list(Directory::Themes.list_stems(), args.json);
}
