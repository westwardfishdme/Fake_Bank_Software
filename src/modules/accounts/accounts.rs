use serde::{Deserialize, Serialize};
use sled::Db;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    balance: f64,
    first_name: String,
    last_name: String,
}

impl Account {
    pub fn new<T: ToString, S: ToString>(first: T, last: S, start_balance: f64) -> Account {
        //! Creates a new account for the user.
        Account {
            first_name: first.to_string(),
            last_name: last.to_string(),
            balance: start_balance,
        }
    }
}

pub fn get_from_database<T: ToString>(username: T, db: &Db) -> Account {
    Account::new(0.to_string(), 0.to_string(), 0_f64)
}
