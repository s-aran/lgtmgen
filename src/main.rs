use ab_glyph::{FontRef, PxScale};
use clap::{command, Parser};
use image::{DynamicImage, GrayImage, Rgb, RgbImage, Rgba};
use imageproc::drawing::draw_text_mut;
extern crate pbr;
extern crate prettytable;

use core::arch;
use std::path::{Path, PathBuf};

fn load_image(path: &PathBuf) -> Result<RgbImage, String> {
    match image::open(path) {
        Ok(d) => return Ok(d.into_rgb8()),
        Err(e) => return Err(e.to_string()),
    };
}

// fn load_font(path: &PathBuf) -> Result<FontRef, String> {
//     let v = match std::fs::read(path) {
//         Ok(d) => d,
//         Err(e) => return Err(e.to_string()),
//     };
//
//     // let r = match Font::try_from_vec(v) {
//     //     Some(f) => f,
//     //     None => return Err(format!("from_vec failed")),
//     // };
//
//     let c = v.clone();
//     let r = match FontRef::try_from_slice(c.as_slice()) {
//         Ok(f) => f,
//         Err(e) => return Err(e.to_string()),
//     };
//
//     Ok(r)
// }

// fn get_glyph(font: &Font, text: &String, scale: f32) {
//     let s = Scale {
//         x: 100.0 * scale,
//         y: 100.0 * scale,
//     };
//     let point = point(0.0, font.v_metrics(s).ascent);
//     let g = font
//         .layout(text, s, point)
//         .map(|g| g.pixel_bounding_box())
//         .filter(|g| g.is_some())
//         .map(|g| g.unwrap())
//         .collect::<Vec<_>>();
// }

fn my_draw_text(
    image: &mut RgbImage,
    text: &str,
    x: i32,
    y: i32,
    font_path: &PathBuf,
    font_size: u32,
    rgb: (u8, u8, u8),
) -> Result<(), String> {
    let v = match std::fs::read(font_path) {
        Ok(d) => d,
        Err(e) => return Err(e.to_string()),
    };

    let c = v.clone();
    let font = match FontRef::try_from_slice(c.as_slice()) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

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

    #[arg(short = 's', long = "size", default_value_t = 10, help = "font size")]
    font_size: u32,

    #[arg(short = 'f', long = "font", help = "path of OTF/TTF font file")]
    font_path: String,

    #[arg(short = 'i', long = "image", help = "path of background image")]
    image_path: String,
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    let source_image = Path::new("./test_images/4.jpg");
    let mut source_image = match load_image(&source_image.to_path_buf()) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let text = "LGTM".to_string();
    let font_path = Path::new("./fonts/roboto/Roboto-Black.ttf");
    match my_draw_text(
        &mut source_image,
        &text,
        100,
        100,
        &font_path.to_path_buf(),
        160,
        (128, 0, 0),
    ) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }

    let dest_image = Path::new("./test_images/4_out.jpg");
    match save_image(&dest_image.to_path_buf(), &source_image) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }
}
