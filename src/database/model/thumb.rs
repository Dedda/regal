use crate::database::model::Picture;
use crate::database::schema::thumbs;

#[derive(Clone, Associations, Identifiable, Queryable, PartialEq, Debug, Insertable)]
#[belongs_to(Picture)]
#[primary_key(picture_id)]
pub struct Thumb {
    pub picture_id: i32,
    pub picture_hash: String,
}
