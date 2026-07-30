use crate::cache::status::{Object, Wallpaper};
use crate::wallpaper;

pub fn handle() {
    if let Some(status) = Wallpaper::try_load() {
        wallpaper::set(status.path, status.mode);
    } else {
        log::warn!("No wallpaper to reload");
    }
}
