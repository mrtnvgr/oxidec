use crate::cache::status::{Object, Wallpaper};
use crate::wallpaper;

pub fn handle() {
    let status = Wallpaper::load();
    wallpaper::set(status.path, status.mode);
}
