use colorsys::{ColorTransform, Rgb};
use upon::Engine;

use super::schema;
use crate::{cache, config};
use std::fs;

pub fn generate(colorscheme: &schema::Colorscheme) {
    let engine = get_engine();

    let templates = config::Folder::Templates.list();
    assert!(!templates.is_empty(), "No templates for generation.");

    for path in templates {
        let content = fs::read_to_string(&path).expect("Failed to read template content");

        let template = engine.compile(&content).unwrap();
        let result = template.render(&engine, colorscheme).to_string().unwrap();

        let template_name: &str = path.file_name().unwrap().to_str().unwrap();
        cache::templates::create(template_name, result).unwrap();
    }
}

fn get_engine() -> Engine<'static> {
    let mut engine = Engine::new();

    engine.add_filter("strip", |text: String| {
        text.trim_start_matches('#').to_owned()
    });

    engine.add_filter("lighten", |text: String, amt: f64| {
        let mut color = Rgb::from_hex_str(&text).unwrap();
        color.lighten(amt);
        color.to_hex_string()
    });

    engine
}
