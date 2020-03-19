use crate::database::{connection, Error};
use diesel::migration::RunMigrationsError;
use crate::database::model::{NewGallery, Gallery, NewPicture, Picture, NewTag, Tag};

embed_migrations!();

impl From<RunMigrationsError> for Error {
    fn from(e: RunMigrationsError) -> Self {
        Error::Unknown(Some(format!("{}", e)))
    }
}

pub fn setup_database() {
    let conn = connection().unwrap();
    embedded_migrations::run(&*conn).unwrap();
    crate::database::provider::gallery::clear_all();
    crate::database::provider::picture::clear_all();
    crate::database::provider::tag::clear_all();
}

pub fn save_gallery(new: &NewGallery) -> Result<Gallery, Error> {
    use crate::database::provider;
    provider::gallery::insert(new)?;
    let mut gallery = provider::gallery::by_name(&new.name)?;
    if !gallery.is_empty() {
        Ok(gallery.remove(0))
    } else {
        Err(Error::Unknown(None))
    }
}

pub fn save_gallery_named(name: &str) -> Result<Gallery, Error> {
    save_gallery(&NewGallery {
        name: name.to_string(),
        directory: None,
        parent: None,
    })
}

pub fn save_picture(new: &NewPicture) -> Result<Picture, Error> {
    use crate::database::provider;
    provider::picture::insert(new)?;
    if let Some(picture) = provider::picture::by_path(&new.path)? {
        Ok(picture)
    } else {
        Err(Error::Unknown(None))
    }
}

pub fn save_tag(new: &NewTag) -> Result<Tag, Error> {
    use crate::database::provider;
    provider::tag::insert(new)?;
    if let Some(tag) = provider::tag::by_name(&new.name)? {
        Ok(tag)
    } else {
        Err(Error::Unknown(None))
    }
}

pub fn save_tag_named(name: &str) -> Result<Tag, Error> {
    save_tag(&NewTag {
        tag_type: 1,
        name: name.to_string(),
    })
}