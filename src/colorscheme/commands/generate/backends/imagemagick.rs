use crate::colorscheme::schema;
use colorsys::{ColorTransform, Rgb, SaturationInSpace};
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use std::process::Command;

pub fn generate(path: &Path, light: bool) -> schema::Colorscheme {
    assert!(
        which::which("convert").is_ok(),
        "ImageMagick is not installed!"
    );

    let colors = generate_colors(path);
    let adjusted_colors = adjust_colors(&colors, light);

    schema::Colorscheme::from_vec_16(&adjusted_colors)
}

fn generate_colors(path: &Path) -> Vec<String> {
    let mut raw_colors = None;

    for i in 0..20 {
        let generated_colors = get_image_colors(path, 16 + i);
        let colors = generated_colors.lines();

        // header + 16 colors
        if colors.count() > 17 {
            break;
        }

        raw_colors = Some(generated_colors);
    }

    if let Some(raw_colors) = raw_colors {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#[0-9a-fA-F]{6}").unwrap();
        }

        let colors: Vec<String> = RE
            .find_iter(&raw_colors)
            .map(|x| x.as_str().to_owned())
            .collect();

        return colors;
    }

    panic!("Imagemagick couldn't generate a suitable palette.");
}

#[allow(clippy::indexing_slicing)]
fn adjust_colors(colors: &[String], light: bool) -> Vec<String> {
    let mut raw_colors = Vec::new();
    raw_colors.extend_from_slice(&colors[0..1]);
    raw_colors.extend_from_slice(&colors[8..16]);
    raw_colors.extend_from_slice(&colors[8..]);
    raw_colors.pop();

    if light {
        for color in &mut raw_colors {
            let mut rgb = Rgb::from_hex_str(color).unwrap();
            rgb.saturate(SaturationInSpace::Hsl(50.0));
            *color = rgb.to_hex_string();
        }

        let mut color15_binding = get_color(colors, 15);
        color15_binding.lighten(85.0);
        raw_colors[0] = color15_binding.to_hex_string();

        let mut color15_binding = get_color(colors, 15);
        color15_binding.lighten(-30.0);
        raw_colors[8] = color15_binding.to_hex_string();

        raw_colors[7] = colors[0].clone();
        raw_colors[15] = colors[0].clone();
    } else {
        if raw_colors[0].starts_with("#0") {
            let mut color0_binding = get_color(&raw_colors, 0);
            color0_binding.lighten(-40.0);
            raw_colors[0] = color0_binding.to_hex_string();
        }

        // TODO: make a pull request to colorsys.rs
        raw_colors[7] = blend_color(&get_color(&raw_colors, 7), "#EEEEEE");

        let mut color7_binding = get_color(&raw_colors, 7);
        color7_binding.lighten(-30.0);
        raw_colors[8] = color7_binding.to_hex_string();

        raw_colors[15] = blend_color(&get_color(&raw_colors, 15), "#EEEEEE");
    }

    raw_colors
}

#[allow(clippy::indexing_slicing)]
fn get_color(colors: &[String], index: usize) -> Rgb {
    Rgb::from_hex_str(&colors[index]).unwrap()
}

fn blend_color(color1: &Rgb, color2: &str) -> String {
    let color2 = Rgb::from_hex_str(color2).unwrap();

    let r3 = 0.5_f64.mul_add(color1.red(), 0.5 * color2.red());
    let g3 = 0.5_f64.mul_add(color1.green(), 0.5 * color2.green());
    let b3 = 0.5_f64.mul_add(color1.blue(), 0.5 * color2.blue());

    Rgb::from((r3, g3, b3)).to_hex_string()
}

fn get_image_colors(path: &Path, color_count: usize) -> String {
    let path = format!("{}[0]", path.display());
    let count = color_count.to_string();

    let output = Command::new("convert")
        .arg(path)
        .args(["-resize", "25%"])
        .args(["-colors", &count])
        .args(["-unique-colors", "txt:-"])
        .output()
        .unwrap();

    String::from_utf8_lossy(&output.stdout).to_string()
}
