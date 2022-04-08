use std::thread::JoinHandle;
use colored::{Color, Colorize};
use crate::database;
use crate::database::model::{Gallery, Picture};
use crate::thumb::generate_if_needed;

const INFO_COLOR: Color = Color::Cyan;
const OK_COLOR: Color = Color::Green;
const ERROR_COLOR: Color = Color::Red;

pub fn launch_background_thumper() -> JoinHandle<()> {
    std::thread::spawn(|| {
        log_info("Background process started.");
        let galleries = load_galleries();
        log_info(&format!("Found {} galleries", galleries.len()));
        for gallery in galleries {
            scan_gallery(gallery);
        }
        log_info("Background process completed.");

    })
}

fn load_galleries() -> Vec<Gallery> {
    database::provider::gallery::all().unwrap()
}

fn scan_gallery(gallery: Gallery) {
    log_info(&format!("Working on gallery: {}", gallery.name));
    let pictures = database::provider::picture::by_gallery(&gallery.id).unwrap();
    log_info(&format!("Found {} pictures", pictures.len()));
    for picture in pictures {
        create_thumbnail(picture);
    }
}

fn create_thumbnail(picture: Picture) {
    match generate_if_needed(&picture) {
        Ok(_) => log_ok(Some('+'), &picture.path),
        Err(e) => log_error(&format!("Error: {:?}", e))
    }
}

fn log_info(text: &str) {
    log(Some('i'), text, INFO_COLOR);
}

fn log_ok(symbol: Option<char>, text: &str) {
    log(symbol, text, OK_COLOR);
}

fn log_error(text: &str) {
    log(Some('e'), text, ERROR_COLOR);
}

fn log(symbol: Option<char>, text: &str, color: Color) {
    let symbol = symbol.unwrap_or(' ');
    println!("  {} [Thumb] {}", symbol.to_string().color(color), text.color(color));
}