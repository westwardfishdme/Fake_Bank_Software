/// This file contains general utilities called throughout the program
/// that do some basic functions pertaining to the database. This acts
/// as an interface to the underlying system that uses sled.
use sled::{self, Db};
/// location of the AUTHENTICATION_DATABASE.
const AUTHENTICATION_DATABASE_PATH: &str = "/tmp/auth_database";
/// location of the USER_DATABASE
const USER_DATABASE_PATH: &str = "/tmp/user_database";

pub fn open_authentication_database() -> Db {
    //! returns the auth_database
    println!("opening auth_database...");
    sled::open(AUTHENTICATION_DATABASE_PATH).unwrap()
}
pub fn open_user_database() -> Db {
    //! returns the auth_database
    println!("opening user_information_database...");
    sled::open(USER_DATABASE_PATH).unwrap()
}
pub trait DatabaseInteractions {
    type ValueStruct;
    fn db_get<T: ToString>(
        username: T,
        db: &Db,
    ) -> Result<Option<Self::ValueStruct>, Box<dyn std::error::Error>>;
    fn db_update(&self, db: &Db) -> Result<(), Box<dyn std::error::Error>>;
    fn db_delete(&self, db: &Db) -> Result<(), Box<dyn std::error::Error>>;
}
