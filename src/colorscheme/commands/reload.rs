use crate::cache::status::{Colorscheme, Object};
use crate::colorscheme::{schema, set_without_cache};

pub fn handle() {
    if let Some(status) = Colorscheme::try_load() {
        let colorscheme = schema::Colorscheme::from_file(status.name, &status.path);
        set_without_cache(&colorscheme);
    } else {
        log::warn!("No colorscheme to reload");
    }
}
