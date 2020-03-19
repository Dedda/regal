pub mod gallery;
pub mod picture;
pub mod tag;
pub mod thumb;
pub mod user;

pub enum InsertStatus {
    Ok,
    AlreadyExists,
}