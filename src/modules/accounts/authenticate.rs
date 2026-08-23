use super::accounts::{self, Account};
use crate::modules::database::utilities::DatabaseInteractions;
use hex::{self, ToHex};
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use sled::Db;
use std::error::Error;

/* general purpose functions */
fn create_salt() -> String {
    let mut chars = [0_u8; 16];
    for i in 0..chars.len() {
        let new_char = rand::random_range(32..=127);
        chars[i] = new_char;
    }
    String::from_utf8_lossy(&chars).to_string()
}
fn hash(password: String) -> String {
    //! creates a password hash from a given string
    Sha256::digest(password.as_bytes()).as_slice().encode_hex()
}
fn create_new_password<T: ToString>(password: T) -> (String, String) {
    //! creates the hash of our salted password, returns both the hash and the salt.
    let salt = create_salt();
    let better_password = format!("{}{salt}", password.to_string());
    (hash(better_password), salt)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
    // for account creation
    first_name: Option<String>,
    last_name: Option<String>,
    new_password: Option<String>,
    init_balance: Option<f64>,
}

impl User {
    pub fn new_password(self, auth_db: &Db) -> Result<(), &'static str> {
        //! changes the password for a user
        let newpass = match self.new_password {
            Some(v) => v,
            None => return Err("Missing 'new_password' field."),
        };
        let mut auth = AuthenticationDetails::db_get(self.username, auth_db)
            .expect("failed to read the database")
            .expect("account did not exist");
        auth.set_new_password(newpass, self.password, auth_db)
    }
    pub fn login(self, auth_db: &Db) -> Result<Option<AuthenticationDetails>, Box<dyn Error>> {
        //! returns the auth db values.
        let stored_binary_values = auth_db.get(self.username)?;
        let db_item: AuthenticationDetails = match stored_binary_values {
            Some(v) => {
                let json = String::from_utf8(v.to_vec())?;
                serde_json::from_str(&json)?
            }
            None => {
                eprintln!("Error in database retrieval; key had no stored values");
                return Ok(None);
            }
        };
        Ok(Some(db_item))
    }

    pub fn create(self, auth_db: &Db, acct_db: &Db) -> Result<(), Box<dyn Error>> {
        //! creates the user in both the account and authentication database.
        let username = self.username;
        let authentication = AuthenticationDetails::create_new_user(&username, self.password);
        let account = Account::create_new_user(
            self.first_name
                .expect("[error] first name was not provided in account creation"),
            self.last_name
                .expect("[error] last name was not provided in account creation"),
            self.init_balance,
        );
        authentication
            .db_create(auth_db)
            .expect("[authentication] failed to create the user in the database");
        account
            .db_create(&username, acct_db)
            .expect("[account] failed to create the user in the database");
        Ok(())
    }
    pub fn username(&self) -> &str {
        return &self.username;
    }

    pub fn password(&self) -> &str {
        return &self.password;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticationDetails {
    /// Username of the account
    user: String,
    /// permission
    permission_id: u8,
    password_hash: String,
    salt: String,
}

/// The authentication database handles user logins,
impl AuthenticationDetails {
    fn create_hash_with_salt<T: ToString>(&self, password: T) -> String {
        //! returns a hash for an inputted password with a given hash value.
        let password = password.to_string();
        hash(format!("{password}{}", self.salt))
    }

    fn create_new_user<T: ToString, U: ToString>(
        username: T,
        new_password: U,
    ) -> AuthenticationDetails {
        //! Creates a new user with user privileges, and submits it to the database
        let (password, salt_value) = create_new_password(new_password.to_string());
        AuthenticationDetails {
            user: username.to_string(),
            password_hash: password,
            salt: salt_value,
            permission_id: 2,
        }
    }
    pub fn set_new_password<T: ToString, U: ToString>(
        &mut self,
        new_password: T,
        old_password: U,
        db: &Db,
    ) -> Result<(), &'static str> {
        //! sets the new password for the user and updates it within the database
        // performs the authentication first here
        let old_hash_result = self.create_hash_with_salt(old_password);
        let new_hash_result = self.create_hash_with_salt(new_password.to_string());
        eprintln!("{} ?= {}", self.password_hash, old_hash_result);
        if old_hash_result != self.password_hash {
            return Err("incorrect password");
        }
        if self.password_hash == new_hash_result {
            return Err("same password was entered");
        }
        // if we are indeed the user; allow the change.
        let (password, salt_value) = create_new_password(new_password.to_string());
        self.password_hash = password;
        self.salt = salt_value;
        self.db_update(db).expect("failed to update the database");
        Ok(())
    }

    pub fn set_perms(&mut self, id: u8) {
        //! lets an admin set the id of a particular user.
        //! 0 -> admin
        //! 1 -> banker
        //! 2 -> user
        self.permission_id = id;
    }

    pub fn get_perms(&self) -> u8 {
        //! get the permission id of the user
        self.permission_id
    }

    pub fn get_password_hash(&self) -> String {
        //! returns the SHA-256 password hash
        self.password_hash.clone()
    }

    pub fn create_admin_account<T: ToString>(password: T, auth_db: &Db) {
        //! Creates an administrative account. If a base 'admin' user does not exist already-- this
        //! function will create the user, other wise it requires an admin account to create.
        if auth_db
            .get("admin")
            .expect("failed to check if an admin account existed in the authentication database")
            .is_none()
        {
            let mut admin = AuthenticationDetails::create_new_user("admin", password);
            admin.set_perms(0);
            admin
                .db_create(auth_db)
                .expect("failed to create the admin acct");
        }
    }

    pub fn authenticate_to_account<T: ToString>(
        &self,
        entered_password: T,
        account_db: &Db,
    ) -> Result<Option<Account>, Box<dyn Error>> {
        //! authenticates a user. if the user doesn't exist, it should fail.

        let hash_result = self.create_hash_with_salt(entered_password.to_string());
        let account = accounts::Account::db_get(self.username(), account_db)?;
        // our return value
        if hash_result == self.password_hash {
            Ok(account)
        } else {
            Ok(None)
        }
    }

    pub fn username(&self) -> String {
        self.user.clone()
    }
    // These require creation to be defined in the actual impl for the struct because Account creation
    // is handled very differently between accounts and authentication. We actually don't want to define
    // this in the trait for this very reason.
    pub fn db_create(&self, db: &Db) -> Result<(), Box<dyn Error>> {
        //! Creates an authentication account in the auth db.
        let json = serde_json::to_string(self)?;
        db.insert(self.username(), json.as_bytes())?;
        Ok(())
    }
}
impl DatabaseInteractions for AuthenticationDetails {
    type ValueStruct = AuthenticationDetails;
    fn db_get<T: ToString>(
        username: T,
        db: &Db,
    ) -> Result<Option<AuthenticationDetails>, Box<dyn Error>> {
        //! gets an object in the authentication database
        // access the stored binary values
        let stored_binary_values = db.get(username.to_string())?;
        let db_item: AuthenticationDetails = match stored_binary_values {
            Some(v) => {
                let json = String::from_utf8(v.to_vec())?;
                serde_json::from_str(&json)?
            }
            None => {
                eprintln!("Error in database retrieval; key had no stored values");
                return Ok(None);
            }
        };
        Ok(Some(db_item))
    }
    fn db_update(&self, db: &Db) -> Result<(), Box<dyn Error>> {
        //! updates an object in the authentication database
        let json = serde_json::to_string(self)?;
        db.remove(self.username())?;
        db.insert(self.username(), json.as_bytes())?;
        Ok(())
    }
    fn db_delete(&self, db: &Db) -> Result<(), Box<dyn Error>> {
        //! deletes an object in the authentication database
        db.remove(self.username())?;
        Ok(())
    }
}

#[cfg(debug_assertions)]
/* Unit tests */
/// These are the unit tests for the authentication system.
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused)]
    fn create_user() -> AuthenticationDetails {
        let username = "john";
        let new_password = String::from("badpassword");
        AuthenticationDetails::create_new_user(username, new_password)
    }
    #[test]
    fn test_setid() {
        const DEFAULT_USER: u8 = 2_u8;
        let mut user_session = create_user();
        // test id setting for ctf
        assert_eq!(DEFAULT_USER, user_session.get_perms());
        user_session.set_perms(0);
        assert_eq!(0, user_session.get_perms(), "setting test failed");
        user_session.set_perms(1);
        assert_eq!(1, user_session.get_perms(), "setting test failed");
    }
    #[test]
    fn test_passwords() {
        let user_session = create_user();
        assert_eq!(user_session.get_password_hash().len(), 64);
    }
    #[test]
    fn test_authentication() {
        let user_session = create_user();
        let user_db = sled::open("/tmp/test_database").unwrap();
        let _ = Account::create_new_user("popqop", "popqop", Some(20_f64))
            .db_create(user_session.username(), &user_db);
        // test authentication
        assert!(
            user_session
                .authenticate_to_account("badpassword", &user_db)
                .expect("failed to authenticate user")
                .is_some()
        );
        assert!(
            user_session
                .authenticate_to_account("badpassword123", &user_db)
                .expect("failed to access account db")
                .is_none()
        );
    }
    #[test]
    fn test_get_username() {
        assert_eq!(create_user().username(), "john");
    }
    #[test]
    fn test_collisons() {
        // test against hash collisions
        let user_session = create_user();
        let user_session2 = create_user();
        assert_ne!(
            user_session.get_password_hash(),
            user_session2.get_password_hash()
        );
    }
    #[test]
    fn test_db() {
        let mut user_session = create_user();
        let db = sled::open("/tmp/test_auth_database/").unwrap();
        match user_session.db_create(&db) {
            Ok(_) => (),
            Err(_) => user_session
                .db_update(&db)
                .expect("failed to update a new db"),
        }
        // test creation
        AuthenticationDetails::db_get("john", &db).expect("failed to retrieve a value from the db");
        // test updating
        assert_eq!(user_session.get_perms(), 2);
        user_session.set_perms(0);
        user_session
            .db_update(&db)
            .expect("failed to update the value");
        let updated_session = AuthenticationDetails::db_get("john", &db)
            .expect("failed to retrieve a value from the db");
        assert_eq!(
            updated_session
                .expect("got none when expected a value")
                .get_perms(),
            0
        );
        // test deletion
        user_session
            .db_delete(&db)
            .expect("failed to delete the value");
        assert!(
            AuthenticationDetails::db_get("john", &db)
                .expect("failed to get a valid result")
                .is_none()
        )
    }
}
