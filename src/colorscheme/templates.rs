use upon::Engine;

use super::schema;
use crate::{cache, config};
use std::fs;

pub fn generate(colorscheme: &schema::Colorscheme) {
    let mut engine = Engine::new();
    engine.add_filter("strip", |text: String| {
        text.trim_start_matches('#').to_owned()
    });

    let templates = config::Folder::Templates.list();

    assert!(!templates.is_empty(), "No templates for generation.");

    for path in templates {
        let content = fs::read_to_string(&path).expect("Failed to read template content");

        let template = engine.compile(&content).unwrap();
        let result = template.render(colorscheme).unwrap();

        let template_name: &str = path.file_name().unwrap().to_str().unwrap();
        cache::templates::create(template_name, result).unwrap();
    }
}
