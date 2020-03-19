use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::SqliteConnection;
use dotenv::dotenv;
use crate::config::Config;
use crate::config;

pub mod model;
pub mod provider;
pub mod schema;

embed_migrations!();

pub type Manager = ConnectionManager<SqliteConnection>;
pub type Result<T> = std::result::Result<T, Error>;

lazy_static! {
    pub static ref POOL: Pool<Manager> = {
        dotenv().ok();
        let database_url = database_url();
        let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
        let pool = Pool::builder().build(manager).expect(&format!("Error opening database"));
        {
            embedded_migrations::run(&pool.get().expect("Cannot get connection for migrations")).expect("Cannot run migrations!");
        }
        pool
    };
}

#[derive(Debug)]
pub enum Error {
    Unknown(Option<String>),
    R2d2(r2d2::Error),
    Diesel(diesel::result::Error),
}

pub fn connection() -> std::result::Result<PooledConnection<Manager>, r2d2::Error> {
    let pool: Pool<Manager> = POOL.clone();
    pool.get()
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error::R2d2(e)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::Diesel(e)
    }
}

#[cfg(not(test))]
fn database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or({
        let conf: &Config = config::get();
        conf.database_file.clone()
    })
}

#[cfg(test)]
fn database_url() -> String {
    "test.sqlite3".to_string()
}

#[cfg(test)]
pub mod tests {
    use crate::database::model::NewGallery;
    use crate::testing::setup_database;
    use crate::database::provider::InsertStatus;

    #[test]
    fn memory_connection() {
        setup_database();
        let galleries = super::provider::gallery::all();
        assert!(galleries.is_ok());
    }

    #[test]
    fn round_trip() {
        use super::provider::gallery;
        setup_database();
        for gal in gallery::all().unwrap() {
            gallery::delete(&gal).unwrap();
        }
        let gal1 = NewGallery {
            name: "Test Gallery 1".to_string(),
            directory: None,
            parent: None,
        };
        let gal2 = NewGallery {
            name: "Test Gallery 2".to_string(),
            directory: Some("/home/test/pics".to_string()),
            parent: None,
        };
        gallery::insert(&gal1).unwrap();
        match gallery::insert(&gal1).unwrap() {
            InsertStatus::AlreadyExists => {},
            _ => panic!("Gallery {} should already exist", &gal1.name),
        }
        gallery::insert(&gal2).unwrap();
        let all = gallery::all().unwrap();
        assert_eq!(2, all.len());
        let gal1 = gallery::by_name(&gal1.name).unwrap();
        assert_eq!("Test Gallery 1", gal1.first().unwrap().name);
        let gal2 = gallery::by_name(&gal2.name).unwrap();
        assert_eq!("Test Gallery 2", gal2.first().unwrap().name);
    }

}