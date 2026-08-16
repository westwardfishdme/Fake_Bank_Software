use crate::database::utilities::open_user_database;
fn new_uuid() -> u64 {
    let database = open_user_database();
    let next_uuid = match database.get(b"total_users") {
        Ok(Some(v)) => String::from_utf8_lossy(v.to_vec().as_slice())
            .parse::<u64>()
            .expect("failed to parse the value into an integer"),
        Err(_) | Ok(None) => {
            // if the value does not exist or the DATABASE_PATH doesn't exist yet:
            database
                .insert(b"total_users", b"0")
                .expect("failed to create a new uuid value");

            let value = match database.get(b"total_users").expect(
                "second database retrieval attempt failed before reaching the match statement",
            ) {
                Some(v) => v,
                None => panic!("Failed to retrieve the uuid value"),
            };
            String::from_utf8_lossy(value.to_vec().as_slice())
                .parse::<u64>()
                .expect("failed to retrieve a new value")
        }
    };
    next_uuid
}
#[derive(Clone, Debug)]
pub struct Account {
    balance: f64,
    first_name: String,
    last_name: String,
    uuid: u64,
}

impl Account {
    pub fn new(first: String, last: String, start_balance: f64) -> Account {
        //! Creates a new account for the user.
        Account {
            first_name: first,
            last_name: last,
            balance: start_balance,
            uuid: new_uuid(),
        }
    }
}
