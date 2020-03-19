use crate::database::{connection, Result};
use crate::database::model::User;
use crate::database::schema::users::dsl::*;

use diesel::prelude::*;

pub fn by_id(user_id: &i32) -> Result<User> {
    let conn = connection()?;
    Ok(users.find(user_id).first::<User>(&*conn)?)
}

pub fn by_username(uname: &str) -> Result<Option<User>> {
    let conn = connection()?;
    let results = users.filter(username.eq(uname)).limit(1).load::<User>(&*conn)?;
    Ok(results.first().map(|a| a.clone()))
}

pub fn by_email(mail: &str) -> Result<Option<User>> {
    let conn = connection()?;
    let results = users.filter(email.eq(mail)).limit(1).load::<User>(&*conn)?;
    Ok(results.first().map(|a| a.clone()))
}