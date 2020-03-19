use crate::database::schema::users;

#[derive(Clone, Associations, Identifiable, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub verification: Option<String>
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub verification: Option<String>
}