use crate::database::model::Picture;

pub fn load_img(image: &Picture) -> std::io::Result<Vec<u8>> {
    let path = &image.path;
    std::fs::read(path)
}

pub fn load_thumb(pic_id: &i32) -> std::io::Result<Vec<u8>> {
    let path = &format!("{}/{}.png", get_thumbs_dir(), pic_id);
    std::fs::read(path)
}

pub fn get_thumbs_dir() -> String {
    format!("{}/thumbs", crate::get_cache_dir())
}