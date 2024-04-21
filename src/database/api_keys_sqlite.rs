use bcrypt::{hash, verify, DEFAULT_COST};
use csv::{Reader, ReaderBuilder, Writer};
use failure::{format_err, Error};
use random_string;
use rusqlite::Connection;
use std::str::FromStr;

const API_KEY_DATABASE: &str = "API_KEYS.sqlite3";

/// Basic level individual permissions
pub enum Permissions {
    Read,
    Write,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Represents possible permissions a user can have
pub enum UserPermission {
    None = 0,
    Reader = 1,
    Writer = 2,
    ReaderWriter = 3,
    Admin = 4,
}

impl UserPermission {
    pub fn to_string(&self) -> String {
        match *self {
            UserPermission::None => "None".to_string(),
            UserPermission::Reader => "Reader".to_string(),
            UserPermission::Writer => "Writer".to_string(),
            UserPermission::ReaderWriter => "ReaderWriter".to_string(),
            UserPermission::Admin => "Admin".to_string(),
        }
    }

    /// Checks if user permission can read
    pub fn read_status(&self) -> bool {
        self.eq(&UserPermission::Reader)
            || self.eq(&UserPermission::ReaderWriter)
            || self.eq(&UserPermission::Admin)
    }

    /// Checks if user permission can write
    pub fn write_status(&self) -> bool {
        self.eq(&UserPermission::Writer)
            || self.eq(&UserPermission::ReaderWriter)
            || self.eq(&UserPermission::Admin)
    }
}

impl FromStr for UserPermission {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Reader" => Ok(UserPermission::Reader),
            "Writer" => Ok(UserPermission::Writer),
            "ReaderWriter" => Ok(UserPermission::ReaderWriter),
            "Admin" => Ok(UserPermission::Admin),
            _ => Ok(UserPermission::None),
        }
    }
}

#[derive(Clone, Debug)]
/// Every entry into the csv looks like this
pub struct Entry {
    user_id: String,
    hashed_api_key: String,
    permission: UserPermission,
}

/// Generates Random API_KEY
fn generate_random_api_key() -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    random_string::generate(12, charset)
}

pub fn init_database() -> Result<(), Error> {
    let conn: Connection = Connection::open(API_KEY_DATABASE)?;

    // Preparing command
    let command = "CREATE TABLE ApiKeyDB (
        user_id    TEXT PRIMARY KEY NOT NULL,
        permission TEXT NOT NULL,
        hashed_key  TEXT NOT NULL
            )";

    // Executing command
    if let Err(e) = conn.execute(command, ()) {
        return Err(format_err!("There was a problem creating table {}", e));
    }

    conn.close().unwrap();

    Ok(())
} 

/// Writes an entry into a csv
fn write_entry(entry: &Entry) -> Result<(), Error> {
    let conn = Connection::open(API_KEY_DATABASE)?;

    // Preparing command
    let command = "INSERT INTO ApiKeyDB (user_id, permission, hashed_key) VALUES (?, ?, ?)";

    // Executing command
    if let Err(e) = conn.execute(
        &command,
        [
            &entry.user_id,
            &entry.permission.to_string(),
            &entry.hashed_api_key,
        ],
    ) {
        return Err(format_err!("There was a problem inserting into table {}", e));
    }

    conn.close().unwrap();

    Ok(())
}

/// Gets entry based on user_id
fn get_entry_by_user_id(user_id: &String) -> Result<Entry, Error> {
    let conn = Connection::open(API_KEY_DATABASE)?;

    // Preparing command
    let mut stmt = conn.prepare(
        "SELECT permission, hashed_key FROM ApiKeyDB WHERE user_id = ?"
    )?;

    let mut rows = stmt.query([user_id])?;

    // Fetch the first row, if any
    if let Some(row) = rows.next()? {
        // Extract data from the row
        let permission: String = row.get(0)?;
        let hashed_key: String = row.get(1)?;

        // Convert permission string to UserPermission enum
        let permission_enum = UserPermission::from_str(&permission)
            .map_err(|_| format_err!("Could not find"))?;

        // Construct and return Entry
        Ok(Entry {
            user_id: user_id.to_string(),
            permission: permission_enum,
            hashed_api_key: hashed_key,
        })
    } else {
        // No entry found for the given user_id
        Err(format_err!("Couldnt find"))
    }
}

pub fn create_user(user_id: &String, permission_level: UserPermission) {
    let random_key = generate_random_api_key();

    let hashed_api_key = hash(random_key, DEFAULT_COST).unwrap();
    let entry = Entry {
        user_id: user_id.clone(),
        hashed_api_key,
        permission: permission_level,
    };

    write_entry(&entry).unwrap();
}

/// Checks if credentials are valid ones and checks permissions
pub fn validate_credentials(
    user_id: &String,
    api_key: &String,
    permission_needed: Permissions,
) -> Result<bool, Error> {
    //create_user(&generate_random_api_key(), UserPermission::Admin);
    let entry = get_entry_by_user_id(user_id)?;

    // Check if api key is a valid api key for user
    if !verify(api_key, &entry.hashed_api_key)? {
        return Err(format_err!("Invalid"));
    }

    match permission_needed {
        Permissions::Read => Ok(entry.permission.read_status()),
        Permissions::Write => Ok(entry.permission.write_status()),
    }
}
