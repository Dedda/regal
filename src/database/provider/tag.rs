use crate::database::{connection, Result};
use crate::database::model::{Tag, NewTag};
use crate::database::schema::tags::dsl::*;
use crate::database::schema::tags::table;

use diesel::prelude::*;

pub fn all() -> Result<Vec<Tag>> {
    let conn = connection()?;
    let results = tags.load::<Tag>(&*conn)?;
    Ok(results)
}

pub fn by_name(tag_name: &str) -> Result<Option<Tag>> {
    let conn = connection()?;
    let results = tags.filter(name.eq(tag_name)).limit(1).load::<Tag>(&*conn)?;
    Ok(results.first().map(|a| a.clone()))
}

pub fn insert(tag: &NewTag) -> Result<()> {
    let conn = connection()?;
    diesel::insert_into(table)
        .values(tag)
        .execute(&*conn)?;
    Ok(())
}

#[cfg(test)]
pub fn clear_all() {
    let conn = connection().unwrap();
    diesel::delete(tags).execute(&*conn).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::testing::{setup_database, save_tag_named};
    use crate::database::model::Tag;

    #[test]
    fn all() {
        setup_database();
        let tags: Vec<Tag> = vec![
            "Tag1",
            "Tag2",
        ].iter().map(|t| save_tag_named(t).unwrap()).collect();
        let loaded = super::all().unwrap();
        assert_eq!(loaded.len(), 2);
        for tag in tags {
            assert!(loaded.contains(&tag));
        }
    }

    #[test]
    fn by_name() {
        setup_database();
        let tags: Vec<Tag> = vec![
            "Tag1",
            "Tag2",
        ].iter().map(|t| save_tag_named(t).unwrap()).collect();
        let loaded = super::by_name("Tag2").unwrap().unwrap();
        assert_eq!(loaded, tags[1])
    }
}