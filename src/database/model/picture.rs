use crate::database::model::Gallery;
use crate::database::schema::pictures;

#[derive(Clone, Associations, Identifiable, Queryable, PartialEq, Debug)]
#[belongs_to(Gallery)]
pub struct Picture {
    pub id: i32,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub gallery_id: i32,
    pub format: String,
    pub path: String,
    pub sha1: String,
    pub filesize: i32,
    pub external_id: String,
}

#[derive(Insertable)]
#[table_name="pictures"]
pub struct NewPicture {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub gallery_id: i32,
    pub format: String,
    pub path: String,
    pub sha1: String,
    pub filesize: i32,
    pub external_id: String,
}