use askama::Template;
use rocket::Rocket;
use rocket::response::status::NotFound;
use crate::database::model::Picture;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/web/picture", routes![by_id])
}

#[derive(Template)]
#[template(path = "web/picture.html")]
struct PicturePage {
    picture_id: i32,
    picture_name: String,
    raw: String,
    gallery: String,
    filename: String,
}

impl From<Picture> for PicturePage {
    fn from(picture: Picture) -> Self {
        let id = picture.id.clone();
        let filename = format!("{}.{}", &picture.name, &picture.format);
        let name = picture.name;
        let raw = format!("/picture/raw/{}", &id);
        PicturePage {
            picture_id: id,
            picture_name: name,
            raw,
            gallery: format!("/web/gallery/{}", &picture.gallery_id),
            filename,
        }
    }
}

#[get("/<id>")]
fn by_id(id: i32) -> Result<PicturePage, NotFound<String>> {
    match crate::database::provider::picture::by_id(&id) {
        Ok(picture) => Ok(picture.into()),
        _ => Err(NotFound(format!("Picture {} not found", &id))),
    }
}
