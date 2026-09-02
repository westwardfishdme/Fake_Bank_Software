use super::transaction::Transaction;
use crate::log;
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use sled::Db;
use std::error::Error;

use crate::modules::database::utilities::DatabaseInteractions;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    username: String,
    balance: f64,
    first_name: String,
    last_name: String,
    transaction_history: Option<Box<[Transaction]>>,
}

impl Account {
    pub fn create_new_user<T: ToString, S: ToString, U: ToString>(
        user: U,
        first: T,
        last: S,
        start_balance: f64,
    ) -> Result<Account, &'static str> {
        //! Creates a new account for the user.
        let username = user.to_string();
        log!("[account] Creating an account for user {}", username);

        if start_balance < 100_f64 {
            log!("[account] User {}", username);
            return Err("User does not have enough money to open account! (value was < $200)");
        }
        Ok(Account {
            username: user.to_string(),
            first_name: first.to_string(),
            last_name: last.to_string(),
            balance: start_balance,
            transaction_history: None,
        })
    }
    pub fn balance(&self) -> f64 {
        //! gets the username associated with the account.
        self.balance
    }

    pub fn get_transaction_history(&self) -> &Option<Box<[Transaction]>> {
        &self.transaction_history
    }

    pub fn append_to_transaction_history(&mut self, transaction: &Transaction) {
        //! appends to the user's transaction_history
        match &self.transaction_history {
            Some(v) => {
                let mut vec_transaction: Vec<Transaction> = (*v).to_vec();
                vec_transaction.push(transaction.clone());
                self.transaction_history = Some(vec_transaction.into_boxed_slice());
                return;
            }
            None => {
                let vec_transaction: Vec<Transaction> = vec![transaction.clone(); 1];
                self.transaction_history = Some(vec_transaction.into_boxed_slice());
            }
        };
    }

    pub fn get_username(&self) -> &String {
        //! gets the username associated with the account.
        &self.username
    }
    pub fn set_new_balance(&mut self, amount: f64) {
        //! sets the new balance from a given amount.
        self.balance = amount;
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
        //! updates an object in the authentication database
        db.remove(self.get_username())?;
        Ok(())
    }
    fn db_update(&self, db: &Db) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(self)?;
        db.remove(self.get_username())?;
        db.insert(self.get_username(), json.as_bytes())?;
        Ok(())
    }
}
