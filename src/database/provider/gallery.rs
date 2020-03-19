use crate::database::{connection, Result};
use crate::database::model::{Gallery, NewGallery};
use crate::database::schema::gallerys::dsl::*;
use crate::database::schema::gallerys::table;

use diesel::prelude::*;
use crate::database::provider::InsertStatus;

pub fn by_id(gallery_id: &i32) -> Result<Gallery> {
    let conn = connection()?;
    Ok(gallerys.find(gallery_id).first::<Gallery>(&*conn)?)
}

pub fn by_name(gallery_name: &str) -> Result<Vec<Gallery>> {
    let conn = connection()?;
    let results = gallerys.filter(name.eq(gallery_name)).load::<Gallery>(&*conn)?;
    Ok(results)
}

pub fn by_directory(dir: &str) -> Result<Option<Gallery>> {
    let conn = connection()?;
    let results = gallerys.filter(directory.eq(dir)).limit(1).load::<Gallery>(&*conn)?;
    Ok(results.first().map(|a| a.clone()))
}

pub fn by_name_and_directory(gallery_name: &str, dir: &Option<String>) -> Result<Option<Gallery>> {
    let conn = connection()?;
    let results = gallerys.filter(directory.eq(dir)).filter(name.eq(gallery_name)).limit(1).load::<Gallery>(&*conn)?;
    Ok(results.first().map(|a| a.clone()))
}

pub fn top_level() -> Result<Vec<Gallery>> {
    let conn = connection()?;
    let results = gallerys.filter(parent.is_null()).load::<Gallery>(&*conn)?;
    Ok(results)
}

pub fn by_parent(parent_id: &i32) -> Result<Vec<Gallery>> {
    let conn = connection()?;
    let results = gallerys.filter(parent.eq(parent_id)).load::<Gallery>(&*conn)?;
    Ok(results)
}

pub fn all() -> Result<Vec<Gallery>> {
    let conn = connection()?;
    let results = gallerys.load::<Gallery>(&*conn)?;
    Ok(results)
}

pub fn insert(gal: &NewGallery) -> Result<InsertStatus> {
    let conn = connection()?;
    if gal.directory.is_none() {
        let found = by_name(&gal.name)?;
        if found.iter().find(|g| g.directory.is_none()).is_some() {
            return Ok(InsertStatus::AlreadyExists);
        }
    }
    if let Some(_gal) = by_name_and_directory(&gal.name.as_str(), &gal.directory)? {
        eprintln!("Gallery {} already exists. Not creating.", &gal.name);
        Ok(InsertStatus::AlreadyExists)
    } else {
        println!("Creating gallery {} [{:?}]", &gal.name, &gal.directory);
        diesel::insert_into(table)
            .values(gal)
            .execute(&*conn)?;
        Ok(InsertStatus::Ok)
    }
}

pub fn delete(gal: &Gallery) -> Result<()> {
    let conn = connection()?;
    diesel::delete(gallerys.find(&gal.id)).execute(&*conn)?;
    Ok(())
}

#[cfg(test)]
pub fn clear_all() {
    let conn = connection().unwrap();
    diesel::delete(gallerys).execute(&*conn).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::database::model::{Gallery, NewGallery};
    use crate::testing::setup_database;

    #[test]
    fn all() {
        setup_database();
        let galleries: Vec<Gallery> = vec![
            "Gal1",
            "Gal2",
        ].iter().map(|g| crate::testing::save_gallery_named(g).unwrap()).collect();
        let loaded = super::all().unwrap();
        assert_eq!(loaded.len(), 2);
        for g in galleries {
            assert!(loaded.contains(&g))
        }
    }

    #[test]
    fn by_id() {
        setup_database();
        let galleries: Vec<Gallery> = vec![
            "Gal1",
            "Gal2",
        ].iter().map(|g| crate::testing::save_gallery_named(g).unwrap()).collect();
        for g in galleries {
            assert_eq!(super::by_id(&g.id).unwrap(), g);
        }
    }

    #[test]
    fn delete() {
        setup_database();
        let gallery = crate::testing::save_gallery_named("Gal1").unwrap();
        assert_eq!(gallery, super::by_id(&gallery.id).unwrap());
        super::delete(&gallery).unwrap();
        match super::by_id(&gallery.id).unwrap_err() {
            crate::database::Error::Diesel(diesel::NotFound) => {},
            _ => assert!(false),
        }
    }

    #[test]
    fn top_level() {
        setup_database();
        let tops: Vec<Gallery> = vec![
            "Gal1",
            "Gal2",
        ].iter().map(|g| crate::testing::save_gallery_named(g).unwrap()).collect();
        super::insert(&NewGallery {
            name: "Child1".to_string(),
            directory: None,
            parent: Some(tops.first().unwrap().id.clone())
        }).unwrap();
        let found = super::top_level().unwrap();
        assert_eq!(found.len(), 2);
        for g in tops {
            assert!(found.contains(&g));
        }
    }

    #[test]
    fn by_parent() {
        setup_database();
        let parent1 = crate::testing::save_gallery_named("Parent 1").unwrap();
        let parent2 = crate::testing::save_gallery_named("Parent 2").unwrap();
        let child11 = crate::testing::save_gallery(&NewGallery {
            name: "Child 1 1".to_string(),
            directory: None,
            parent: Some(parent1.id.clone())
        }).unwrap();
        let child12 = crate::testing::save_gallery(&NewGallery {
            name: "Child 1 2".to_string(),
            directory: None,
            parent: Some(parent1.id.clone())
        }).unwrap();
        let child21 = crate::testing::save_gallery(&NewGallery {
            name: "Child 2 1".to_string(),
            directory: None,
            parent: Some(parent2.id.clone())
        }).unwrap();
        let found = super::by_parent(&parent1.id).unwrap();
        assert_eq!(found.len(), 2);
        assert!(found.contains(&child11));
        assert!(found.contains(&child12));
        assert!(!found.contains(&child21));
    }

}