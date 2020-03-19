#![feature(proc_macro_hygiene, decl_macro)]

extern crate askama;
#[macro_use]
extern crate clap;
extern crate colored;
extern crate ctrlc;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dirs;
extern crate dotenv;
extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_sqlite;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rsgen;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha;
extern crate signal_hook;
extern crate uuid;

use colored::Colorize;
use crate::config::{Config, ScanDir};
use std::path::Path;
use std::process::exit;
use clap::{App, ArgMatches};

pub mod auth;
pub mod config;
pub mod database;
pub mod disk;
mod net;
pub mod scan;
pub mod thumb;
#[cfg(test)]
pub mod testing;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref ARGS: MyArgs = {
        let yaml = load_yaml!("clap.yml");
        App::from_yaml(yaml).get_matches().into()
    };
}

pub struct MyArgs {
    pub config: Option<String>,
    pub cache: Option<String>,
    pub skip_scan: bool,
    pub skip_thumbs: bool,
}

impl<'a> From<ArgMatches<'a>> for MyArgs {
    fn from(a: ArgMatches<'a>) -> Self {
        Self {
            config: a.value_of("config").map(ToString::to_string),
            cache: a.value_of("cache").map(ToString::to_string),
            skip_scan: a.is_present("skip scan"),
            skip_thumbs: a.is_present("skip thumbs"),
        }
    }
}

fn main() {
    ctrlc::set_handler(|| {
        println!("Received CTRL+C. Stopping");
        exit(0);
    }).expect("Cannot register CTRL+C handle");
    unsafe {
        signal_hook::register(signal_hook::SIGTERM, || {
            println!("Received SIGTERM. Stopping");
            exit(0);
        }).expect("Cannot register SIGTERM handle");
    }

    get_cache_dir();
    println!("Regal v{}", VERSION);
    let conf: &Config = config::get();
    if !ARGS.skip_scan {
        for dir in conf.scan_dirs.iter() {
            init_gallery(dir);
        }

        println!("\n{}\n===========\n", "Checking existing galleries".blue());
        for gallery in database::provider::gallery::all().unwrap() {
            scan::check_gallery(&gallery.id).unwrap();
        }
    }

    if !ARGS.skip_thumbs {
        println!("\n{}\n===========\n", "Generating thumbnails".blue());
        for gallery in database::provider::gallery::all().unwrap() {
            println!("{} [{}]", "Generating thumbnails in gallery".blue(), gallery.name.magenta());
            for pic in database::provider::picture::by_gallery(&gallery.id).unwrap() {
                thumb::generate_if_needed(&pic).unwrap();
            }
        }
    }

    net::launch();
}

fn init_gallery(dir: &ScanDir) {
    if dir.recursive {
        scan::scan_recursively(&dir.path, &vec![]).unwrap();
    } else {
        scan::scan(dir, None).unwrap();
    }
}

pub fn get_cache_dir() -> String {
    let mut cache = dirs::cache_dir().map(|p| p.to_str().unwrap().to_string());
    if cache.is_none() {
        cache = dirs::home_dir().map(|p| p.to_str().unwrap().to_string());
    }

    // Check if cache dir is overridden by args
    if let Some(c) = ARGS.cache.clone() {
        cache = Some(c);
    }

    let mut cache = Path::new(&cache.unwrap()).to_path_buf();
    cache.push(".regal");
    if !cache.exists() {
        std::fs::create_dir(cache.clone()).unwrap();
        let mut thumbs = cache.clone();
        thumbs.push("thumbs");
        std::fs::create_dir(thumbs).unwrap();
    }
    cache.to_str().unwrap().to_string()
}