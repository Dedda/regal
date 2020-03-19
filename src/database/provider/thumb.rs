use crate::database::{connection, Result};
use crate::database::model::Thumb;
use crate::database::schema::thumbs::dsl::*;
use crate::database::schema::thumbs::table;

use diesel::prelude::*;
use crate::database::provider::InsertStatus;

pub fn all() -> Result<Vec<Thumb>> {
    let conn = connection()?;
    let results = thumbs.load::<Thumb>(&*conn)?;
    Ok(results)
}

pub fn by_picture(p_id: &i32) -> Result<Option<Thumb>> {
    let conn = connection()?;
    let results = thumbs.filter(picture_id.eq(p_id)).limit(1).load::<Thumb>(&*conn)?;
    Ok(results.first().map(|a| a.clone()))
}

pub fn update(t_id: &i32, thmb: &Thumb) -> Result<()> {
    let conn = connection()?;
    diesel::update(thumbs.find(t_id))
        .set(picture_hash.eq(&thmb.picture_hash))
        .execute(&*conn)?;
    Ok(())
}

pub fn insert(thmb: &Thumb) -> Result<InsertStatus> {
    let conn = connection()?;
    if by_picture(&thmb.picture_id)?.is_some() {
        return Ok(InsertStatus::AlreadyExists)
    }
    diesel::insert_into(table)
        .values(thmb)
        .execute(&*conn)?;
    Ok(InsertStatus::Ok)
}

pub fn delete(thmb: &Thumb) -> Result<()> {
    let conn = connection()?;
    diesel::delete(thumbs.find(&thmb.picture_id)).execute(&*conn)?;

    Ok(())
}
