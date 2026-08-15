use crate::common;
use crate::{config::Directory, wallpaper::args};

pub fn handle(args: &args::List) {
    common::handle_list(Directory::Wallpapers.list_names(), args.json);
}
