use serde::{Deserialize, Serialize};
use sled::Db;
use std::error::Error;
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

pub fn db_get<T: ToString>(username: T, account_db: &Db) -> Result<Option<Account>, &'static str> {
    //! gets the user's account information
    let stored_binary_values = account_db
        .get(username.to_string())
        .expect("failed to access the database");
    let db_item = match stored_binary_values {
        Some(v) => {
            let json = String::from_utf8(v.to_vec()).expect("failed to convert to valid utf-8");
            serde_json::from_str(&json).expect("json is not stored correctly in the account db")
        }
        None => {
            eprintln!(
                "[account] Error in database retrieval for {}; key had no stored values",
                username.to_string()
            );
            return Err("failed to get a valid user in the account database");
        }
    };
    Ok(db_item)
}
