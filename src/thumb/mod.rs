use crate::database::model::{Thumb, Picture};
use crate::disk::get_thumbs_dir;
use colored::Colorize;
use image::ImageError;
use image::imageops::FilterType::Triangle;

#[derive(Debug)]
pub enum ThumbError {
    Database(crate::database::Error),
    Image(ImageError),
    Io(std::io::Error),
}

impl From<crate::database::Error> for ThumbError {
    fn from(e: crate::database::Error) -> Self {
        ThumbError::Database(e)
    }
}

impl From<ImageError> for ThumbError {
    fn from(e: ImageError) -> Self {
        ThumbError::Image(e)
    }
}

impl From<std::io::Error> for ThumbError {
    fn from(e: std::io::Error) -> Self {
        ThumbError::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, ThumbError>;

pub fn load(pic_id: &i32) -> Result<Option<Vec<u8>>> {
    if crate::database::provider::thumb::by_picture(pic_id)? .is_some(){
        Ok(Some(crate::disk::load_thumb(pic_id)?))
    } else {
        Ok(None)
    }
}

pub fn load_or_generate(pic: &Picture) -> Result<Option<Vec<u8>>> {
    if let Some(thumb) = load(&pic.id)? {
        Ok(Some(thumb))
    } else {
        generate(&pic)?;
        load(&pic.id)
    }
}

pub fn generate_if_needed(pic: &Picture) -> Result<()> {
    let thumb = crate::database::provider::thumb::by_picture(&pic.id)?;
    if thumb.is_none() || thumb.unwrap().picture_hash.ne(&pic.sha1) {
        generate(&pic)
    } else {
        Ok(())
    }
}

pub fn generate(pic: &Picture) -> Result<()> {
    let img = image::open(&pic.path)?;
    let img = img.resize(100, 100, Triangle);
    let path = thumb_path(&pic.id);
    println!("  {} [Thumb] {}", "+".green(), path.green());
    img.save(path)?;
    let thumb = Thumb {
        picture_id: pic.id.clone(),
        picture_hash: pic.sha1.clone(),
    };
    crate::database::provider::thumb::insert(&thumb)?;
    Ok(())
}

fn thumb_path(pic_id: &i32) -> String {
    format!("{}/{}.png", get_thumbs_dir(), pic_id)
}
