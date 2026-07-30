use colorsys::{ColorTransform, Rgb};
use upon::Engine;

use super::schema;
use crate::{cache, config};
use std::fs;

pub fn generate(colorscheme: &schema::Colorscheme) {
    let engine = get_engine();

    let templates = config::Directory::Templates.list();
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

    engine.add_function("strip", |text: String| {
        text.trim_start_matches('#').to_owned()
    });

    engine.add_function("r", |text: String| {
        let color = Rgb::from_hex_str(&text).unwrap();
        color.red()
    });

    engine.add_function("g", |text: String| {
        let color = Rgb::from_hex_str(&text).unwrap();
        color.green()
    });

    engine.add_function("b", |text: String| {
        let color = Rgb::from_hex_str(&text).unwrap();
        color.blue()
    });

    engine.add_function("css_rgb", |text: String| {
        let color = Rgb::from_hex_str(&text).unwrap();
        color.to_css_string()
    });

    engine.add_function("lighten", |text: String, amt: f64| {
        let mut color = Rgb::from_hex_str(&text).unwrap();
        color.lighten(amt);
        color.to_hex_string()
    });

    engine
}

#[cfg(test)]
mod test {
    use colorsys::Rgb;

    /// `to_css_string` should always return RGB
    #[test]
    fn test_css_rgb() {
        let color = Rgb::from_hex_str("#FF0000").unwrap();
        assert_eq!(color.to_css_string(), "rgb(255,0,0)");

        let color = Rgb::from_hex_str("#00FF0030").unwrap();
        assert_eq!(color.to_css_string(), "rgba(0,255,0,0.19)");
    }
}
