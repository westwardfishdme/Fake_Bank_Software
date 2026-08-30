use super::accounts::Account;
use crate::log;
use crate::modules::database::utilities::DatabaseInteractions;
use serde::{Deserialize, Serialize};
use sled::Db;

pub enum TransferFailures {
    FailedToSend(&'static str),
    AccountDoesNotExist(&'static str),
    InsufficientFunds(&'static str),
    NoTransactionHistory(&'static str),
    DatabaseFailure(&'static str),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    from: String,
    recipient: String,
    amount: f64,
}
pub trait Transfers {
    fn send<T: ToString>(
        &mut self,
        recipient: T,
        amount: f64,
        db: &Db,
    ) -> Result<(), TransferFailures>;
    fn reverse(&mut self, db: &Db) -> Result<(), TransferFailures>;
}

impl Transaction {
    pub fn new<T: ToString, V: ToString>(sent_from: T, send_to: V, amt: f64) -> Transaction {
        //! Creates a new transaction query.
        Transaction {
            from: sent_from.to_string(),
            recipient: send_to.to_string(),
            amount: amt,
        }
    }
    pub fn get_sender(&self) -> &String {
        //! gets the sender of the transaction
        &self.from
    }
    pub fn get_recipient(&self) -> &String {
        //! gets the recipient of the transaction.
        &self.recipient
    }
    pub fn get_amount(&self) -> f64 {
        //! gets the amount that was sent
        self.amount
    }
}

impl Transfers for Account {
    fn send<T: ToString>(
        &mut self,
        recipient: T,
        amount: f64,
        account_database: &Db,
    ) -> Result<(), TransferFailures> {
        //! Implements a transaction for the Account.
        let recipient = &recipient.to_string();
        let transaction = Transaction::new(&self.get_username(), recipient, amount);
        // pre-conditions:
        // 1. User cannot send to themselves.
        // 2. User cannot send if the amount requested is less than what is in their balance.
        // 3. recipient must be a real account.
        if self.get_username() == recipient {
            return Err(TransferFailures::FailedToSend(
                "Attempted to send the amount to self.",
            ));
        }
        if self.balance() < amount {
            return Err(TransferFailures::InsufficientFunds("Insufficient Funds"));
        }
        // ensure that the user is a real account
        let db_get_result = match Account::db_get(recipient, account_database) {
            Ok(v) => v,
            Err(_) => {
                log!(
                    "[account] failed to perform transaction: account database failed to be read."
                );
                return Err(TransferFailures::FailedToSend(
                    "Failed to access the given database.",
                ));
            }
        };
        let mut recipient = match db_get_result {
            Some(v) => v,
            None => {
                log!(
                    "[account] failed to perform transaction: account database did not have user {}.",
                    recipient
                );
                return Err(TransferFailures::AccountDoesNotExist(
                    "Requested account did not exist.",
                ));
            }
        };
        // subtract from the sender's account, and then append the amount to the recipient.

        let new_balance = self.balance() - amount;
        self.set_new_balance(new_balance);
        match self.db_update(account_database) {
            Ok(()) => (),
            Err(_) => {
                log!(
                    "[account] Failed to update the user database for user {}",
                    self.get_username()
                );
                return Err(TransferFailures::FailedToSend(
                    "Failed to conduct the transaction-- internal error.",
                ));
            }
        }

        let new_balance = recipient.balance() + amount;
        self.append_to_transaction_history(transaction);

        recipient.set_new_balance(new_balance);
        match recipient.db_update(account_database) {
            Ok(()) => Ok(()),
            Err(_) => {
                log!("Failed to update database");
                Err(TransferFailures::DatabaseFailure(
                    "failed to update database",
                ))
            }
        }
    }
    fn reverse(&mut self, db: &Db) -> Result<(), TransferFailures> {
        //! reverses the transaction history
        match self.get_transaction_history() {
            Some(v) => {
                let last_idx = v.len() - 1;
                let last_transaction = v.get(last_idx).unwrap();

                // get the transaction information
                let recipient = last_transaction.get_recipient();
                let sender = last_transaction.get_sender();
                let amount = last_transaction.get_amount();

                // reverse the transaction.
                match Account::db_get(recipient, db).unwrap() {
                    Some(mut v) => v.send(sender, amount, db),
                    None => {
                        log!("[transaction] reversal failed, recipient {} did not exist.");
                        Err(TransferFailures::AccountDoesNotExist(
                            "recipient did not exist",
                        ))
                    }
                }
            }
            None => Err(TransferFailures::NoTransactionHistory(
                "User had no transaction history.",
            )),
        }
    }
}
