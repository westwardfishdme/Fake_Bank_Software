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
pub trait DatabaseInteractions<T: ToString, S: ToString> {
    type ValueStruct;
    fn create(username: T, password: S) -> Result<(), Box<dyn std::error::Error>>;
    fn get_from_database(u: T) -> Result<Self::ValueStruct, Box<dyn std::error::Error>>;
    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn delete(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
