use std::path::Path;
use std::fs::DirEntry;
use crate::database::model::{NewPicture, NewGallery};
use serde::export::fmt::Debug;
use crate::ScanDir;
use sha::utils::{Digest, DigestExt};
use image::{ImageError, GenericImageView};
use colored::Colorize;
use uuid::Uuid;

static FORMATS: [&'static str; 8] = ["png", "jpg", "jpeg", "gif", "bmp", "ico", "tiff", "webp"];

#[derive(Debug)]
pub enum ScanError {
    UnknownFormat(String),
    Io(std::io::Error),
    Database(crate::database::Error),
    Image(ImageError),
}

impl From<std::io::Error> for ScanError {
    fn from(e: std::io::Error) -> Self {
        ScanError::Io(e)
    }
}

impl From<crate::database::Error> for ScanError {
    fn from(e: crate::database::Error) -> Self {
        ScanError::Database(e)
    }
}

impl From<ImageError> for ScanError {
    fn from(e: ImageError) -> Self {
        ScanError::Image(e)
    }
}

pub type ScanResult<T> = Result<T, ScanError>;

pub fn scan(scan_dir: &ScanDir, parent: Option<i32>) -> ScanResult<()> {
    use crate::database::provider;
    let dir = &scan_dir.path;
    let gallery_id = match crate::database::provider::gallery::by_directory(dir).unwrap() {
        Some(gallery) => gallery.id,
        None => {
            let name = name_from_path(dir);
            crate::database::provider::gallery::insert(&NewGallery {
                name,
                directory: Some(dir.to_string()),
                parent,
            })?;
            crate::database::provider::gallery::by_directory(dir).unwrap().unwrap().id
        }
    };
    let gallery = crate::database::provider::gallery::by_id(&gallery_id)?;

    let found = files_in_directory(dir)?;

    for file in found {
        let img = provider::picture::by_path(&file)?;
        if img.is_some() && img.clone().unwrap().filesize.eq(&(std::fs::metadata(&file)?.len() as i32)) {
            continue;
        }
        let sha1 = sha::sha1::Sha1::default().digest(&std::fs::read(Path::new(&file)).unwrap()).to_hex();
        if img.is_none() || img.unwrap().sha1.ne(&sha1) {
            match scan_picture(&file, &gallery_id, sha1) {
                Ok(img) => {
                    println!("{} [{}] {}", "+".green(), gallery.name.green(), img.name.green());
                    provider::picture::insert(&img)?;
                },
                Err(_) => eprintln!("{} [{}]", "! Error scanning file:".yellow(), file.yellow()),
            }
        }
    }
    Ok(())
}

pub fn scan_recursively(scan_dir: &str, parents: &Vec<String>) -> ScanResult<()> {
    use crate::database::provider;
    let picture_files = files_in_directory(scan_dir)?;
    if !picture_files.is_empty() {
        create_parents(scan_dir, parents)?;
        let gallery = provider::gallery::by_directory(scan_dir)?.unwrap();
        for picture_file in picture_files {
            let img = provider::picture::by_path(&picture_file)?;
            if img.is_some() && img.clone().unwrap().filesize.eq(&(std::fs::metadata(&picture_file)?.len() as i32)) {
                continue;
            }
            let sha1 = sha::sha1::Sha1::default().digest(&std::fs::read(Path::new(&picture_file)).unwrap()).to_hex();
            if img.is_none() || img.unwrap().sha1.ne(&sha1) {
                let name = name_from_path(&picture_file);
                println!("{} [{}] {}", "+".green(), gallery.name.green(), name.green());
                if let Ok(new_picture) = scan_picture(&picture_file, &gallery.id, sha1) {
                    provider::picture::insert(&new_picture)?;
                } else {
                    eprintln!("{} {}", "! Error scanning file:".yellow(), picture_file.yellow());
                }
            }
        }
    }
    let sub_dirs = directories_in_directory(scan_dir)?;
    if !sub_dirs.is_empty() {
        let mut new_parents = parents.clone();
        new_parents.push(scan_dir.to_string());
        for sub_dir in sub_dirs {
            println!("Checking dir {}", &sub_dir);
            scan_recursively(&sub_dir, &new_parents)?;
        }
    }
    Ok(())
}

fn create_parents(dir: &str, parents: &Vec<String>) -> ScanResult<()> {
    use crate::database::provider;
    let mut last: Option<i32> = None;
    for parent in parents.iter() {
        if provider::gallery::by_directory(parent)?.is_none() {
            let  name = name_from_path(parent);
            println!("{} [{}]", "+".green(),  parent.green());
            provider::gallery::insert(&NewGallery {
                name,
                directory: Some(parent.clone()),
                parent: last
            })?;
        }
        last = Some(provider::gallery::by_directory(&parent)?.unwrap().id);
    }
    if provider::gallery::by_directory(dir)?.is_none() {
        println!("{} [{}]", "+".green(), dir.green());
        let name = name_from_path(dir);
        provider::gallery::insert(&NewGallery {
            name,
            directory: Some(dir.to_string()),
            parent: last
        })?;
    }
    Ok(())
}

fn name_from_path(path: &str) -> String {
    Path::new(path).file_name().unwrap().to_str().unwrap().to_string()
}

fn files_in_directory(dir: &str) -> ScanResult<Vec<String>> {
    let dir = Path::new(dir);
    let dir: Vec<DirEntry> = dir.read_dir()?.filter_map(Result::ok).collect();
    Ok(
        dir.into_iter()
            .filter(|d| d.file_type().unwrap().is_file())
            .map(|d| d.path().to_str().unwrap().to_string())
            .filter(|p| {
                if let Some(extension) = Path::new(p).extension() {
                    FORMATS.contains(&extension.to_str().unwrap().to_lowercase().as_str())
                } else {
                    false
                }
            })
            .collect()
    )
}

fn directories_in_directory(dir: &str) -> ScanResult<Vec<String>> {
    let dir = Path::new(dir);
    let dir: Vec<DirEntry> = dir.read_dir()?.filter_map(Result::ok).collect();
    Ok(dir.into_iter().filter(|d| d.file_type().unwrap().is_dir()).map(|d| d.path().to_str().unwrap().to_string()).collect())
}

pub fn check_gallery(gallery_id: &i32) -> Result<(), crate::database::Error> {
    let gallery = crate::database::provider::gallery::by_id(gallery_id)?;
    if let Some(path) = gallery.directory.clone() {
        let path = Path::new(&path);
        if !path.exists() {
            crate::database::provider::gallery::delete(&gallery)?;
            return Ok(());
        }
    }
    let pictures = crate::database::provider::picture::by_gallery(gallery_id)?;
    for picture in pictures {
        let path = Path::new(&picture.path);
        if !path.exists() {
            println!("{} [{}] {}", "-".red(), gallery.name.red(), &picture.name.red());
            crate::database::provider::picture::delete(&picture)?;
        }
    }
    Ok(())
}

fn scan_picture(file: &str, gallery_id: &i32, sha1: String) -> ScanResult<NewPicture> {
    let path = Path::new(file);
    let name = path.file_stem().unwrap().to_str().unwrap().to_string();
    let picture = image::open(path)?;
    let width = picture.width() as i32;
    let height = picture.height() as i32;
    let format = path.extension().unwrap().to_str().unwrap().to_lowercase();
    let filesize = std::fs::metadata(path)?.len() as i32;
    let external_id = format!("{}.{}", Uuid::new_v4().to_simple().encode_lower(&mut Uuid::encode_buffer()), &format);
    Ok(NewPicture {
        name,
        width,
        height,
        gallery_id: gallery_id.clone(),
        format,
        path: file.to_string(),
        sha1,
        filesize,
        external_id,
    })
}