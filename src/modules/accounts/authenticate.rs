use super::accounts::{self, Account};
use hex::{self, ToHex};
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use sled::Db;
use std::error::Error;

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

fn create_new_password(password: String) -> (String, String) {
    //! creates the hash of our salted password, returns both the hash and the salt.
    let salt = create_salt();
    let better_password = format!("{password}{salt}");
    (hash(better_password), salt)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Authenticate {
    username: String,
    password: String,
}

impl Authenticate {
    pub fn login(self, auth_db: &Db) -> Result<Option<AuthenticationDatabase>, Box<dyn Error>> {
        //! returns the auth db values.
        let stored_binary_values = auth_db.get(self.username)?;
        let db_item: AuthenticationDatabase = match stored_binary_values {
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
    pub fn create(self, auth_db: &Db) {
        AuthenticationDatabase::create_new_user(self.username, self.password)
            .db_create(auth_db)
            .expect("failed to create the user in the database");
    }
    pub fn password(&self) -> &str {
        return &self.password;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticationDatabase {
    /// Username of the account
    user: String,
    /// permission
    permission_id: u8,
    password_hash: String,
    salt: String,
}

impl AuthenticationDatabase {
    fn empty() -> AuthenticationDatabase {
        // exploit lies here.
        AuthenticationDatabase {
            user: String::new(),
            permission_id: 0,
            password_hash: String::new(),
            salt: String::new(),
        }
    }
    fn create_new_user<T: ToString, U: ToString>(
        username: T,
        new_password: U,
    ) -> AuthenticationDatabase {
        //! Creates a new user with user privileges, and submits it to the database
        let (password, salt_value) = create_new_password(new_password.to_string());
        AuthenticationDatabase {
            user: username.to_string(),
            password_hash: password,
            salt: salt_value,
            permission_id: 2,
        }
    }

    pub fn set_perms(&mut self, id: u8) {
        //! lets an admin set the id of a particular user.
        //! 0 -> admin
        //! 1 -> banker
        //! 2 -> user
        self.permission_id = id;
    }

    pub fn get_perms(&self) -> u8 {
        //! get the id of the user
        self.permission_id
    }

    pub fn get_password_hash(&self) -> String {
        //! returns the SHA-256 password hash
        self.password_hash.clone()
    }

    fn create_hash_with_salt(&self, password: String) -> String {
        //! returns a hash for an inputted password with a given hash value.
        hash(format!("{password}{}", self.salt))
    }
    pub fn authenticate<T: ToString>(
        &self,
        entered_password: T,
        account_db: &Db,
    ) -> Result<Option<Account>, Box<dyn Error>> {
        //! authenticates a user. if the user doesn't exist, it should fail.

        let hash_result = self.create_hash_with_salt(entered_password.to_string());
        let account = accounts::db_get(self.username(), account_db)?;
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

    // database
    fn db_create(&self, db: &Db) -> Result<(), Box<dyn Error>> {
        //! creates an authentication account in the auth db
        let json = serde_json::to_string(self)?;
        db.insert(self.username(), json.as_bytes())?;
        Ok(())
    }

    fn db_get<T: ToString>(
        username: T,
        db: &Db,
    ) -> Result<Option<AuthenticationDatabase>, Box<dyn Error>> {
        //! gets an object in the authentication database
        // access the stored binary values
        let stored_binary_values = db.get(username.to_string())?;
        let db_item: AuthenticationDatabase = match stored_binary_values {
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
    fn db_delete(&self, db: &Db) -> Result<(), Box<dyn std::error::Error>> {
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
    fn create_user() -> AuthenticationDatabase {
        let username = "john";
        let new_password = String::from("badpassword");
        AuthenticationDatabase::create_new_user(username, new_password)
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
        let user_db = crate::modules::database::utilities::open_user_database();

        // test authentication
        assert!(user_session.authenticate("badpassword", &user_db).is_some());
        assert!(
            user_session
                .authenticate("badpassword123", &user_db)
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
        let db = crate::modules::database::utilities::open_authentication_database();
        match user_session.db_create(&db) {
            Ok(_) => (),
            Err(_) => user_session.update(&db).expect("failed to update a new db"),
        }
        // test creation
        AuthenticationDatabase::db_get("john", &db)
            .expect("failed to retrieve a value from the db");
        // test updating
        assert_eq!(user_session.get_perms(), 2);
        user_session.set_perms(0);
        user_session
            .update(&db)
            .expect("failed to update the value");
        let updated_session = AuthenticationDatabase::db_get("john", &db)
            .expect("failed to retrieve a value from the db");
        assert_eq!(
            updated_session
                .expect("got none when expected a value")
                .get_perms(),
            0
        );
        // test deletion
        user_session
            .delete(&db)
            .expect("failed to delete the value");
        assert!(
            AuthenticationDatabase::db_get("john", &db)
                .expect("failed to get a valid result")
                .is_none()
        )
    }
}
