use rocket::Rocket;
use crate::database::model::{Gallery, NewGallery};
use rocket_contrib::json::Json;
use rocket::response::status::{NotFound, BadRequest};
use rocket::request::Form;

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/gallery", routes![all, by_id, top_level, by_parent, list_all, create, delete])
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct GalleryData {
    gallery_id: i32,
    gallery_name: String,
    picture_list: String,
    display: String,
    thumb: String,
}

impl From<Gallery> for GalleryData {
    fn from(gal: Gallery) -> Self {
        let mut thumb = "none".to_string();
        if let Ok(Some(img)) = crate::database::provider::picture::find_thumb(&gal.id) {
            thumb = format!("/picture/thumb/{}", &img.id);
        }
        GalleryData {
            gallery_id: gal.id.clone(),
            gallery_name: gal.name.clone(),
            picture_list: format!("/picture/in_gallery/{}", &gal.id),
            display: format!("/web/gallery/{}", &gal.id),
            thumb,
        }
    }
}

#[derive(FromForm)]
struct NewGalleryForm {
    name: String,
    directory: Option<String>,
    parent: Option<i32>
}

impl Into<NewGallery> for NewGalleryForm {
    fn into(self) -> NewGallery {
        let NewGalleryForm {
            name,
            directory,
            parent
        } = self;
        NewGallery {
            name,
            directory,
            parent
        }
    }
}

#[get("/list_all")]
fn list_all() {
    let galleries = crate::database::provider::gallery::all().unwrap();
    let json = serde_json::to_string(&galleries).unwrap();
    println!("\n\n\n{}\n\n\n", json);
}

#[get("/all")]
fn all() -> Result<Json<Vec<GalleryData>>, NotFound<&'static str>> {
    if let Ok(galleries) = crate::database::provider::gallery::all() {
        Ok(Json(galleries.into_iter().map(|g| g.into()).collect()))
    } else {
        Err(NotFound("Error loading gallery list"))
    }
}

#[get("/<gallery_id>")]
fn by_id(gallery_id: i32) -> Result<Json<GalleryData>, NotFound<String>> {
    if let Ok(gallery) = crate::database::provider::gallery::by_id(&gallery_id) {
        Ok(Json(gallery.into()))
    } else {
        Err(NotFound(format!("Gallery {} not found.", gallery_id)))
    }
}

#[get("/top")]
fn top_level() -> Result<Json<Vec<GalleryData>>, NotFound<String>> {
    if let Ok(galleries) = crate::database::provider::gallery::top_level() {
        Ok(Json(galleries.into_iter().map(|g| g.into()).collect()))
    } else {
        Err(NotFound(format!("Error loading galleries")))
    }
}

#[get("/by_parent/<parent_id>")]
fn by_parent(parent_id: i32) -> Result<Json<Vec<GalleryData>>, NotFound<String>> {
    if let Ok(galleries) = crate::database::provider::gallery::by_parent(&parent_id) {
        Ok(Json(galleries.into_iter().map(|g| g.into()).collect()))
    } else {
        Err(NotFound(format!("Error loading galleries")))
    }
}

#[post("/new", data = "<new_gallery>")]
fn create(new_gallery: Form<NewGalleryForm>) -> Result<(), BadRequest<String>> {
    match crate::database::provider::gallery::insert(&new_gallery.into_inner().into()) {
        Ok(_) => Ok(()),
        Err(e) => Err(BadRequest(Some(format!("Error creating gallery: {:?}", e))))
    }
}

#[delete("/<gallery_id>")]
fn delete(gallery_id: i32) -> Result<(), BadRequest<String>> {
    if let Ok(gallery) = crate::database::provider::gallery::by_id(&gallery_id) {
        match crate::database::provider::gallery::delete(&gallery) {
            Ok(_) => Ok(()),
            Err(e) => Err(BadRequest(Some(format!("Error deleting gallery: {:?}", e))))
        }
    } else {
        Err(BadRequest(Some(format!("Gallery [{}] does not exists or could not be loaded", &gallery_id))))
    }
}

#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use crate::net::gallery::GalleryData;
    use crate::database::model::NewGallery;
    use rocket::local::Client;

    fn setup() -> Client {
        crate::testing::setup_database();
        super::super::test_client()
    }

    #[test]
    fn all() {
        let client = setup();
        let galleries = vec![
            crate::testing::save_gallery(&NewGallery {
                name: "Gal1".to_string(),
                directory: None,
                parent: None,
            }).unwrap(),
            crate::testing::save_gallery(&NewGallery {
                name: "Gal2".to_string(),
                directory: Some("/home/test/pics".to_string()),
                parent: None,
            }).unwrap(),
        ];
        let mut response = client.get("/gallery/all").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let resp_string = response.body_string().unwrap();
        let parsed: Vec<GalleryData> = serde_json::from_str(&resp_string).unwrap();
        assert_eq!(parsed.len(), galleries.len());
        for gallery in galleries {
            assert!(parsed.contains(&gallery.into()));
        }
    }

    #[test]
    fn by_id() {
        let client = setup();
        let galleries = vec![
            crate::testing::save_gallery(&NewGallery {
                name: "Gal1".to_string(),
                directory: None,
                parent: None,
            }).unwrap(),
            crate::testing::save_gallery(&NewGallery {
                name: "Gal2".to_string(),
                directory: Some("/home/test/pics".to_string()),
                parent: None,
            }).unwrap(),
        ];
        assert_eq!(galleries.len(), 2);
        for gallery in galleries {
            let id = &gallery.id;
            let mut response = client.get(format!("/gallery/{}", id)).dispatch();
            assert_eq!(response.status(), Status::Ok);
            let parsed: GalleryData = serde_json::from_str(&response.body_string().unwrap()).unwrap();
            assert_eq!(parsed, gallery.into());
        }
    }
}