use rocket::Rocket;
use rocket_contrib::json::Json;
use rocket::response::status::NotFound;
use crate::database::model::Picture;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/picture", routes![data, raw, thumb, in_gallery])
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PictureData {
    picture_id: i32,
    picture_name: String,
    raw: String,
    thumb: String,
    display: String,
}

impl From<Picture> for PictureData {
    fn from(img: Picture) -> Self {
        let name = img.name.clone();
        let id = img.id;
        PictureData {
            picture_id: id,
            picture_name: name,
            raw: format!("/picture/raw/{}", id),
            thumb: format!("/picture/thumb/{}", id),
            display: format!("/web/picture/{}", id),
        }
    }
}

#[get("/data/<img_id>")]
fn data(img_id: i32) -> Result<Json<PictureData>, NotFound<String>> {
    if let Ok(picture) = crate::database::provider::picture::by_id(&img_id) {
        Ok(Json(picture.into()))
    } else {
        Err(NotFound(format!("Picture with id {} was not found.", img_id)))
    }
}

#[get("/raw/<img_id>")]
fn raw(img_id: i32) -> Result<Vec<u8>, NotFound<String>> {
    if let Ok(picture) = crate::database::provider::picture::by_id(&img_id) {
        if let Ok(data) = crate::disk::load_img(&picture) {
            Ok(data)
        } else {
            Err(NotFound(format!("File '{}' was not found.", &picture.path)))
        }
    } else {
        Err(NotFound(format!("Picture with id {} was not found.", img_id)))
    }
}

#[get("/thumb/<img_id>")]
fn thumb(img_id: i32) -> Result<Vec<u8>, NotFound<String>> {
    if let Ok(picture) = crate::database::provider::picture::by_id(&img_id) {
        if let Ok(data) = crate::disk::load_thumb(&img_id) {
            Ok(data)
        } else {
            Err(NotFound(format!("File '{}' was not found.", &picture.path)))
        }
    } else {
        Err(NotFound(format!("Picture with id {} was not found.", img_id)))
    }
}

#[get("/in_gallery/<gallery_id>")]
fn in_gallery(gallery_id: i32) -> Result<Json<Vec<PictureData>>, NotFound<String>> {
    if let Ok(gallery) = crate::database::provider::gallery::by_id(&gallery_id) {
        if let Ok(pictures) = crate::database::provider::picture::by_gallery(&gallery.id) {
            let pictures = pictures.into_iter().map(Into::into).collect();
            Ok(Json(pictures))
        } else {
            Err(NotFound(format!("Pictures for gallery {} not found.", gallery.id)))
        }
    } else {
        Err(NotFound(format!("Gallery {} not found.", gallery_id)))
    }
}

#[cfg(test)]
mod tests {
    use crate::database::model::{Picture, NewPicture, NewGallery};
    use crate::net::picture::PictureData;
    use rocket::local::Client;
    use rocket::http::Status;

    fn setup() -> Client {
        crate::testing::setup_database();
        super::super::test_client()
    }

    #[test]
    fn picture_data_from_picture() {
        let picture = Picture {
            id: 123,
            name: "IMG_0001.png".to_string(),
            width: 0,
            height: 0,
            gallery_id: 3,
            format: "png".to_string(),
            path: "/home/test/IMG_0001.png".to_string(),
            sha1: "".to_string(),
            filesize: 0,
        };
        let picture_data: PictureData = picture.into();
        assert_eq!(&123, &picture_data.picture_id);
        assert_eq!("IMG_0001.png", &picture_data.picture_name);
        assert_eq!("/picture/raw/123", &picture_data.raw);
        assert_eq!("/picture/thumb/123", &picture_data.thumb);
    }

    #[test]
    fn data() {
        let client = setup();
        let gallery = crate::testing::save_gallery(&NewGallery {
            name: "Gal1".to_string(),
            directory: None,
            parent: None,
        }).unwrap();
        let picture = crate::testing::save_picture(&NewPicture {
            name: "Img1".to_string(),
            width: 0,
            height: 0,
            gallery_id: gallery.id,
            format: "png".to_string(),
            path: "/1.png".to_string(),
            sha1: "".to_string(),
            filesize: 0,
        }).unwrap();
        let mut response = client.get(format!("/picture/data/{}", &picture.id)).dispatch();
        let parsed: PictureData = serde_json::from_str(&response.body_string().unwrap()).unwrap();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(parsed, picture.into());
    }

    #[test]
    fn in_gallery() {
        let client = setup();
        let gallery = crate::testing::save_gallery(&NewGallery {
            name: "Gal1".to_string(),
            directory: None,
            parent: None,
        }).unwrap();
        let wrong_gallery = crate::testing::save_gallery(&NewGallery {
            name: "Gal2".to_string(),
            directory: None,
            parent: None,
        }).unwrap();
        let pictures = vec![
            crate::testing::save_picture(&NewPicture {
                name: "Img1".to_string(),
                width: 0,
                height: 0,
                gallery_id: gallery.id.clone(),
                format: "png".to_string(),
                path: "/home/test/img1.png".to_string(),
                sha1: "".to_string(),
                filesize: 0,
            }).unwrap(),
            crate::testing::save_picture(&NewPicture {
                name: "Img2".to_string(),
                width: 0,
                height: 0,
                gallery_id: gallery.id.clone(),
                format: "png".to_string(),
                path: "/home/test/img2.png".to_string(),
                sha1: "".to_string(),
                filesize: 0,
            }).unwrap(),
            crate::testing::save_picture(&NewPicture {
                name: "Img3".to_string(),
                width: 0,
                height: 0,
                gallery_id: wrong_gallery.id.clone(),
                format: "png".to_string(),
                path: "/home/test/img3.png".to_string(),
                sha1: "".to_string(),
                filesize: 0,
            }).unwrap(),
        ];
        let mut response = client.get(format!("/picture/in_gallery/{}", &gallery.id)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let parsed: Vec<PictureData> = serde_json::from_str(&response.body_string().unwrap()).unwrap();
        assert_eq!(parsed.len(), 2);
        for picture in pictures.into_iter().filter(|i| i.name.ne("Img3")) {
            assert!(parsed.contains(&picture.into()));
        }
    }
}