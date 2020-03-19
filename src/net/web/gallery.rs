use askama::Template;
use rocket::Rocket;
use rocket::response::status::NotFound;
use crate::database::model::Gallery;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/web/gallery", routes![by_id, new])
}

#[derive(Template)]
#[template(path = "web/gallery.html", escape = "none")]
struct GalleryPage {
    gallery_name: String,
    pictures: String,
    sub_galleries: String,
    parent: String,
}

impl From<Gallery> for GalleryPage {
    fn from(gal: Gallery) -> Self {
        GalleryPage {
            gallery_name: gal.name.clone(),
            pictures: format!("/picture/in_gallery/{}", &gal.id),
            sub_galleries: format!("/gallery/by_parent/{}", &gal.id),
            parent: if let Some(parent) = gal.parent {
                format!("/gallery/{}", parent)
            } else {
                String::new()
            }
        }
    }
}

#[get("/<gallery_id>")]
fn by_id(gallery_id: i32) -> Result<GalleryPage, NotFound<String>> {
    if let Ok(gallery) = crate::database::provider::gallery::by_id(&gallery_id) {
        Ok(gallery.into())
    } else {
        Err(NotFound(format!("Gallery {} not found.", gallery_id)))
    }
}

#[derive(Template)]
#[template(path = "web/gallery_new.html")]
struct NewGalleryPage {}

#[get("/new")]
fn new() -> NewGalleryPage {
    NewGalleryPage {}
}