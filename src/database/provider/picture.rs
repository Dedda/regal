use crate::database::{connection, Result};
use crate::database::model::{Picture, NewPicture};
use crate::database::schema::pictures::dsl::*;
use crate::database::schema::pictures::table;

use diesel::prelude::*;
use crate::database::provider::InsertStatus;

pub fn by_id(picture_id: &i32) -> Result<Picture> {
    let conn = connection()?;
    Ok(pictures.find(picture_id).first::<Picture>(&*conn)?)
}

pub fn by_path(img_path: &str) -> Result<Option<Picture>> {
    let conn = connection()?;
    let results = pictures.filter(path.eq(img_path)).limit(1).load::<Picture>(&*conn)?;
    Ok(results.first().map(|a| a.clone()))
}

pub fn by_gallery(g_id: &i32) -> Result<Vec<Picture>> {
    let conn = connection()?;
    Ok(pictures.filter(gallery_id.eq(g_id)).load::<Picture>(&*conn)?)
}

pub fn find_thumb(g_id: &i32) -> Result<Option<Picture>> {
    use super::gallery;
    let imgs = by_gallery(&g_id)?;
    if let Some(img) = imgs.first() {
        return Ok(Some(img.clone()));
    }
    let gals = gallery::by_parent(g_id)?;
    for gal in gals {
        if let Some(img) = find_thumb(&gal.id)? {
            return Ok(Some(img));
        }
    }
    Ok(None)
}

pub fn update(img_id: &i32, img: &NewPicture) -> Result<()> {
    let conn = connection()?;
    diesel::update(pictures.find(img_id))
        .set((name.eq(&img.name), width.eq(&img.width), height.eq(&img.height), sha1.eq(&img.sha1)))
        .execute(&*conn)?;
    Ok(())
}

pub fn insert(img: &NewPicture) -> Result<InsertStatus> {
    use super::gallery;
    gallery::by_id(&img.gallery_id)?;
    let conn = connection()?;
    if let Some(_img) = by_path(&img.path.as_str())? {
        Ok(InsertStatus::AlreadyExists)
    } else {
        diesel::insert_into(table)
            .values(img)
            .execute(&*conn)?;
        Ok(InsertStatus::Ok)
    }
}

pub fn delete(img: &Picture) -> Result<()> {
    let conn = connection()?;
    diesel::delete(pictures.find(&img.id)).execute(&*conn)?;

    Ok(())
}

#[cfg(test)]
pub fn clear_all() {
    let conn = connection().unwrap();
    diesel::delete(pictures).execute(&*conn).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::testing::{setup_database, save_picture, save_gallery_named};
    use crate::database::model::{Gallery, Picture, NewPicture};

    #[test]
    fn by_id_by_path() {
        setup_database();
        let gallery = save_gallery_named("Gal1").unwrap().id;
        let pictures: Vec<Picture> = vec![
            NewPicture {
                name: "Img1".to_string(),
                width: 0,
                height: 0,
                gallery_id: gallery.clone(),
                format: "png".to_string(),
                path: "/img1.png".to_string(),
                sha1: "".to_string(),
                filesize: 0,
            },
            NewPicture {
                name: "Img2".to_string(),
                width: 0,
                height: 0,
                gallery_id: gallery.clone(),
                format: "jpg".to_string(),
                path: "/img2.jpg".to_string(),
                sha1: "".to_string(),
                filesize: 0,
            }
        ].iter().map(|i| save_picture(i).unwrap()).collect();
        for img in pictures {
            assert_eq!(&super::by_id(&img.id).unwrap(), &img);
            assert_eq!(&super::by_path(&img.path).unwrap().unwrap(), &img);
        }
    }

    #[test]
    fn by_gallery() {
        setup_database();
        let galleries: Vec<Gallery> = vec![
            "Gal1",
            "Gal2",
        ].iter().map(|g| save_gallery_named(g).unwrap()).collect();
        let right_gallery = galleries.get(0).unwrap().id;
        let wrong_gallery = galleries.get(1).unwrap().id;
        let pictures: Vec<Picture> = vec![
            NewPicture {
                name: "Pic1".to_string(),
                width: 0,
                height: 0,
                gallery_id: right_gallery,
                format: "png".to_string(),
                path: "/p1.png".to_string(),
                sha1: "".to_string(),
                filesize: 0,
            },
            NewPicture {
                name: "Pic2".to_string(),
                width: 0,
                height: 0,
                gallery_id: wrong_gallery,
                format: "png".to_string(),
                path: "/p2.png".to_string(),
                sha1: "".to_string(),
                filesize: 0,
            },
            NewPicture {
                name: "Pic3".to_string(),
                width: 0,
                height: 0,
                gallery_id: right_gallery,
                format: "jpg".to_string(),
                path: "/p3.jpg".to_string(),
                sha1: "".to_string(),
                filesize: 0,
            }
        ].iter().map(|i| save_picture(i).unwrap()).collect();
        let loaded = super::by_gallery(&right_gallery).unwrap();
        assert_eq!(loaded.len(), 2);
        for img in pictures.iter().filter(|i| i.gallery_id.eq(&right_gallery)) {
            assert!(loaded.contains(img));
        }
    }
}