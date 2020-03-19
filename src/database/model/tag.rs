use crate::database::model::Picture;
use crate::database::schema::{picture_tags, tags};

#[derive(Clone, Associations, Identifiable, Queryable, PartialEq, Debug)]
pub struct Tag {
    pub id: i32,
    pub tag_type: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="tags"]
pub struct NewTag {
    pub tag_type: i32,
    pub name: String,
}

#[derive(Clone, Associations, Insertable, Queryable, PartialEq, Debug)]
#[belongs_to(Picture)]
#[belongs_to(Tag)]
pub struct PictureTag {
    tag_id: i32,
    picture_id: i32,
}