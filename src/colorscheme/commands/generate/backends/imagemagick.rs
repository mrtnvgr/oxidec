use colorsys::{ColorTransform, Rgb, SaturationInSpace};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::path::Path;
use std::process::Command;

pub fn generate(path: &Path, light: bool) -> Vec<Rgb> {
    assert!(
        which::which("convert").is_ok(),
        "ImageMagick is not installed!"
    );

    let colors = generate_colors(path);
    adjust_colors(&colors, light)
}

fn generate_colors(path: &Path) -> Vec<Rgb> {
    let mut raw_colors = None;

    for i in 0..20 {
        let generated_colors = get_image_colors(path, 16 + i);
        let colors = generated_colors.lines();

        // header + 16 colors
        match colors.count().cmp(&17) {
            Ordering::Less => continue,
            Ordering::Greater => break,
            Ordering::Equal => raw_colors = Some(generated_colors),
        }
    }

    if let Some(raw_colors) = raw_colors {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#[0-9a-fA-F]{6}").unwrap();
        }

        let colors: Vec<Rgb> = RE
            .find_iter(&raw_colors)
            .map(|x| Rgb::from_hex_str(x.as_str()).unwrap())
            .collect();

        return colors;
    }

    panic!("Imagemagick couldn't generate a suitable palette.");
}

#[allow(clippy::indexing_slicing)]
fn adjust_colors(colors: &[Rgb], light: bool) -> Vec<Rgb> {
    let mut raw_colors = Vec::new();
    raw_colors.extend_from_slice(&colors[0..1]);
    raw_colors.extend_from_slice(&colors[8..16]);
    raw_colors.extend_from_slice(&colors[8..]);
    raw_colors.pop();

    if light {
        for color in &mut raw_colors {
            color.saturate(SaturationInSpace::Hsl(50.0));
        }

        raw_colors[0] = colors[15].clone();
        raw_colors[0].lighten(85.0);

        raw_colors[8] = colors[15].clone();
        raw_colors[8].lighten(-30.0);

        raw_colors[7] = colors[0].clone();
        raw_colors[15] = colors[0].clone();
    } else {
        if raw_colors[0].to_hex_string().starts_with("#0") {
            raw_colors[0].lighten(-40.0);
        }

        lazy_static! {
            static ref GRAY: Rgb = Rgb::from_hex_str("#EEEEEE").unwrap();
        }

        raw_colors[7] = blend_color(&raw_colors[7], &GRAY);

        raw_colors[8] = raw_colors[7].clone();
        raw_colors[8].lighten(-30.0);

        raw_colors[15] = blend_color(&raw_colors[15], &GRAY);
    }

    raw_colors
}

fn blend_color(color1: &Rgb, color2: &Rgb) -> Rgb {
    let r3 = 0.5_f64.mul_add(color1.red(), 0.5 * color2.red());
    let g3 = 0.5_f64.mul_add(color1.green(), 0.5 * color2.green());
    let b3 = 0.5_f64.mul_add(color1.blue(), 0.5 * color2.blue());

    Rgb::from((r3, g3, b3))
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
