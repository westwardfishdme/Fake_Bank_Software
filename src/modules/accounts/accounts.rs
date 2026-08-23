use super::transaction::{TransferFailures, Transfers};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::error::Error;

use crate::modules::database::utilities::DatabaseInteractions;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    balance: f64,
    first_name: String,
    last_name: String,
    transaction_history: Option<Box<[Account]>>,
}

impl Account {
    pub fn create_new_user<T: ToString, S: ToString>(
        first: T,
        last: S,
        start_balance: Option<f64>,
    ) -> Account {
        //! Creates a new account for the user.
        let unwrapped_balance = if start_balance.is_some() {
            start_balance.unwrap()
        } else {
            0_f64
        };
        Account {
            first_name: first.to_string(),
            last_name: last.to_string(),
            balance: unwrapped_balance,
            transaction_history: None,
        }
    }
    pub fn db_create<T: ToString>(
        &self,
        username: T,
        account_db: &Db,
    ) -> Result<(), Box<dyn Error>> {
        //! creates the user's account information
        let json = serde_json::to_string(self)?;
        account_db.insert(username.to_string(), json.as_bytes())?;
        Ok(())
    }
}

impl DatabaseInteractions for Account {
    type ValueStruct = Account;
    fn db_get<T: ToString>(
        username: T,
        account_db: &Db,
    ) -> Result<Option<Account>, Box<dyn Error>> {
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
                    "[account] Error in user database retrieval for {}; key had no stored values",
                    username.to_string()
                );
                return Err("failed to get a valid user in the account database".into());
            }
        };
        Ok(db_item)
    }
    fn db_delete(&self, db: &Db) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn db_update(&self, db: &Db) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
impl Transfers for Account {
    fn send<T: ToString>(&self, recipient: T) -> Result<(), TransferFailures> {
        Ok(())
    }
    fn request<T: ToString>(&self, recipient: T) -> Result<(), TransferFailures> {
        Ok(())
    }
    fn reverse(&self) -> Result<(), TransferFailures> {
        Ok(())
    }
}
