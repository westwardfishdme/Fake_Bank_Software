use std::error::Error;

use crate::database::utilities::{DatabaseInteractions, open_authentication_database};

use hex::{self, ToHex};
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};

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
pub struct Authentication {
    /// Username of the account
    user: String,
    /// permission
    permission_id: u8,
    password_hash: String,
    salt: String,
}

impl Authentication {
    fn empty() -> Authentication {
        // exploit lies here.
        Authentication {
            user: String::new(),
            permission_id: 0,
            password_hash: String::new(),
            salt: String::new(),
        }
    }
    pub fn create_new_user<T: ToString, U: ToString>(
        username: T,
        new_password: U,
    ) -> Authentication {
        //! Creates a new user with user privileges
        let (password, salt_value) = create_new_password(new_password.to_string());
        Authentication {
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
    pub fn authenticate<T: ToString>(&self, entered_password: T) -> bool {
        //! authenticates a user. returns:
        //! - true on success
        //! - false on fail

        let hash_result = self.create_hash_with_salt(entered_password.to_string());
        // our return value
        hash_result == self.password_hash
    }
    pub fn username(&self) -> String {
        self.user.clone()
    }
}
impl<T: ToString, S: ToString> DatabaseInteractions<T, S> for Authentication {
    type ValueStruct = Authentication;
    fn create(username: T, password: S) -> Result<(), Box<dyn Error>> {
        //! creates an object in the authentication database
        let db = open_authentication_database();
        let new_user = Authentication::create_new_user(username.to_string(), password.to_string());
        let json = serde_json::to_string(&new_user)?;
        db.insert(username.to_string().as_bytes(), json.as_bytes())?;
        Ok(())
    }
    fn get_from_database(username: T) -> Result<Self::ValueStruct, Box<dyn Error>> {
        //! gets an object in the authentication database
        let db = open_authentication_database();
        // access the stored binary values
        let stored_binary_values = db.get(username.to_string())?;
        let db_item: Authentication = match stored_binary_values {
            Some(v) => {
                let json = String::from_utf8(v.to_vec())?;
                serde_json::from_str(&json)?
            }
            None => {
                eprintln!("Error in database retrieval; returning empty auth struct");
                return Ok(Authentication::empty());
            }
        };
        Ok(db_item)
    }
    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        //! updates an object in the authentication database
        let db = open_authentication_database();
        let json = serde_json::to_string(self)?;
        db.remove(self.username())?;
        db.insert(self.username(), json.as_bytes())?;
        Ok(())
    }
    fn delete(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        //! deletes an object in the authentication database
        let db = open_authentication_database();
        let json = serde_json::to_string(self)?;
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
    fn create_user() -> Authentication {
        let username = "john";
        let new_password = String::from("badpassword");
        Authentication::create_new_user(username, new_password)
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
        // test authentication
        assert!(user_session.authenticate("badpassword"));
        assert!(!user_session.authenticate("badpassword123"));
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
}
