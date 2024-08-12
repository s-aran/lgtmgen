use ab_glyph::{Font, FontRef};
use clap::{command, Parser};
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use regex::bytes::Regex;

use std::path::{Path, PathBuf};

fn load_image(path: &PathBuf) -> Result<RgbImage, String> {
    match image::open(path) {
        Ok(d) => return Ok(d.into_rgb8()),
        Err(e) => return Err(e.to_string()),
    };
}

fn load_font(path: &PathBuf) -> Result<Vec<u8>, String> {
    match std::fs::read(path) {
        Ok(d) => return Ok(d),
        Err(e) => return Err(e.to_string()),
    };
}

fn get_font(font_data: &Vec<u8>) -> Result<FontRef, String> {
    let s = font_data.as_slice();
    let font = match FontRef::try_from_slice(s) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    Ok(font)
}

fn my_draw_text(
    image: &mut RgbImage,
    text: &str,
    x: i32,
    y: i32,
    font: &FontRef,
    font_size: u16,
    rgb: RgbColor,
) -> Result<(), String> {
    let scale = ab_glyph::PxScale {
        x: (font_size) as f32,
        y: font_size as f32,
    };

    let mut cm = image.clone();

    draw_text_mut(
        &mut cm,
        Rgb([rgb.0, rgb.1, rgb.2]),
        x,
        y,
        scale,
        &font,
        &text,
    );

    image.clone_from(&cm);

    Ok(())
}

fn save_image(path: &PathBuf, image: &RgbImage) -> Result<(), String> {
    match image.save(path) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e.to_string()),
    };
}

#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short = 't', long = "text", default_value = "LGTM", help = "text")]
    text: String,

    #[arg(
        short = 'x',
        default_value = "auto",
        help = "X coordinate, if specific offset value, insert  +/- to first"
    )]
    x: String,
    #[arg(
        short = 'y',
        default_value = "auto",
        help = "Y coordinate, if specific offset value, insert  +/- to first"
    )]
    y: String,

    #[arg(
        short = 'c',
        long = "color",
        default_value = "#FFFFFF",
        help = "color #HEX"
    )]
    color: String,

    #[arg(short = 's', long = "size", default_value_t = 200, help = "font size")]
    font_size: u16,

    #[arg(short = 'f', long = "font", help = "path of OTF/TTF font file")]
    font_path: String,

    #[arg(short = 'i', long = "image", help = "path of background image")]
    image_path: String,

    #[arg(short = 'o', long = "output", help = "path of output image")]
    output_path: String,
}

type RgbColor = (u8, u8, u8);

fn convert_color_string_to_rgb(color: &String) -> Result<RgbColor, String> {
    static RGB_PATTERN_6: &str = "^#?([0-9a-f]{6})$";
    static RGB_PATTERN_3: &str = "^#?([0-9a-f]{3})$";

    let re_6 = Regex::new(RGB_PATTERN_6).unwrap();
    let re_3 = Regex::new(RGB_PATTERN_3).unwrap();

    let lowered = color.to_lowercase();

    {
        let captures = re_6.captures(&lowered.as_bytes());
        if captures.is_some() {
            let c = captures.unwrap();

            if c.get(1).unwrap().len() == 6 {
                let raw_bytes = c.get(1).unwrap().to_owned().as_bytes();

                let maybe_r = std::str::from_utf8(&raw_bytes[0..2]);
                let maybe_g = std::str::from_utf8(&raw_bytes[2..4]);
                let maybe_b = std::str::from_utf8(&raw_bytes[4..]);

                let r = match maybe_r {
                    Ok(d) => u8::from_str_radix(d, 16).unwrap_or(0),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                let g = match maybe_g {
                    Ok(d) => u8::from_str_radix(d, 16).unwrap_or(0),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                let b = match maybe_b {
                    Ok(d) => u8::from_str_radix(d, 16).unwrap_or(0),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                return Ok((r, g, b));
            }
        }
    }

    {
        let captures = re_3.captures(&lowered.as_bytes());
        if captures.is_some() {
            let c = captures.unwrap();

            if c.get(1).unwrap().len() == 3 {
                let raw_bytes = c.get(1).unwrap().to_owned().as_bytes();

                let maybe_r = std::str::from_utf8(&raw_bytes[0..1]);
                let maybe_g = std::str::from_utf8(&raw_bytes[1..2]);
                let maybe_b = std::str::from_utf8(&raw_bytes[2..]);

                let r = match maybe_r {
                    Ok(d) => u8::from_str_radix(format!("{}{}", d, d).as_str(), 16).unwrap_or(0),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                let g = match maybe_g {
                    Ok(d) => u8::from_str_radix(format!("{}{}", d, d).as_str(), 16).unwrap_or(0),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                let b = match maybe_b {
                    Ok(d) => u8::from_str_radix(format!("{}{}", d, d).as_str(), 16).unwrap_or(0),
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                return Ok((r, g, b));
            }
        }
    }

    let rgb = match color.as_str() {
        "black" => (0, 0, 0),
        "white" => (255, 255, 255),
        "red" => (255, 0, 0),
        "green" => (0, 255, 0),
        "blue" => (0, 0, 255),
        "yellow" => (255, 255, 0),
        "cyan" => (0, 255, 255),
        "magenta" => (255, 0, 255),
        _ => return Err(format!("invalid color {}", color)),
    };

    Ok(rgb)
}

fn calc_center(text: &String, font_size: u16, font: &FontRef, image: &RgbImage) -> (i32, i32) {
    let id_list = text.chars().map(|c| font.glyph_id(c));
    let glyphs = id_list
        .filter_map(|id| font.glyph_raster_image2(id, font_size))
        .collect::<Vec<_>>();

    let max_width = glyphs.iter().map(|g| g.width).max().unwrap_or(0);
    let max_height = glyphs.iter().map(|g| g.height).max().unwrap_or(0);
    let min_width = glyphs.iter().map(|g| g.width).min().unwrap_or(0);
    let min_height = glyphs.iter().map(|g| g.height).min().unwrap_or(0);

    let width_diff = max_width - min_width;
    let height_diff = max_height - min_height;

    let center_x = (image.width() as i32 - width_diff as i32) / 2;
    let center_y = (image.height() as i32 - height_diff as i32) / 2;

    (center_x, center_y)
}

fn main() {
    let args = Args::parse();

    let color = match convert_color_string_to_rgb(&args.color) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let source_image = Path::new(&args.image_path);
    let mut source_image = match load_image(&source_image.to_path_buf()) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let font_path = Path::new(&args.font_path).to_path_buf();
    let font_data = match load_font(&font_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let font = match get_font(&font_data) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let text = args.text;
    let (x, y) = calc_center(&text, args.font_size, &font, &source_image);

    match my_draw_text(&mut source_image, &text, x, y, &font, args.font_size, color) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }

    let dest_image = Path::new(&args.output_path);
    match save_image(&dest_image.to_path_buf(), &source_image) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::convert_color_string_to_rgb;

    #[test]
    fn test_color_6() {
        {
            let color = "#FF0F00".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((255, 15, 0)));
        }

        {
            let color = "00FF0F".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((0, 255, 15)));
        }
    }

    #[test]
    fn test_color_3() {
        {
            let color = "#F10".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((255, 17, 0)));
        }

        {
            let color = "0F1".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((0, 255, 17)));
        }
    }

    #[test]
    fn test_color_name() {
        {
            let color = "black".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((0, 0, 0)));
        }
        {
            let color = "white".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((255, 255, 255)));
        }
        {
            let color = "red".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((255, 0, 0)));
        }
        {
            let color = "green".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((0, 255, 0)));
        }
        {
            let color = "blue".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((0, 0, 255)));
        }
        {
            let color = "yellow".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((255, 255, 0)));
        }
        {
            let color = "cyan".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((0, 255, 255)));
        }
        {
            let color = "magenta".to_string();
            let rgb = convert_color_string_to_rgb(&color);
            assert_eq!(rgb, Ok((255, 0, 255)));
        }
    }
}
