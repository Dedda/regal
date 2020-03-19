use crate::database::schema::gallerys;

#[derive(Clone, Associations, Identifiable, Queryable, PartialEq, Debug, Serialize)]
pub struct Gallery {
    pub id: i32,
    pub name: String,
    pub directory: Option<String>,
    pub parent: Option<i32>,
}

#[derive(Insertable)]
#[table_name="gallerys"]
pub struct NewGallery {
    pub name: String,
    pub directory: Option<String>,
    pub parent: Option<i32>,
}