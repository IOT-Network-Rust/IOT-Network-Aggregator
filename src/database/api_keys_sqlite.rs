use bcrypt::{hash, verify, DEFAULT_COST};
use csv::{Reader, Writer, ReaderBuilder};
use failure::{format_err, Error};
use std::str::FromStr;
use random_string;
use rayon::prelude::*;


const API_KEY_CSV: &str = "API_KEYS.csv";

/// Basic level individual permissions
pub enum Permissions {
    Read,
    Write,
}

#[derive(PartialEq, Eq)]
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
        self.eq(&UserPermission::Reader) ||
        self.eq(&UserPermission::ReaderWriter) ||
        self.eq(&UserPermission::Admin)
    }

    /// Checks if user permission can write
    pub fn write_status(&self) -> bool {
        self.eq(&UserPermission::Writer) ||
        self.eq(&UserPermission::ReaderWriter) ||
        self.eq(&UserPermission::Admin)
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

/// Writes an entry into a csv
fn write_entry(entry: &Entry) -> Result<(), Error> {
    let mut writer = Writer::from_path(API_KEY_CSV)
        .map_err(|e| format_err!("Failed to open CSV file: {}", e))?;
    writer
        .write_record(&[
            &entry.user_id,
            &entry.hashed_api_key,
            &entry.permission.to_string(),
        ])
        .map_err(|e| format_err!("Could not insert data: {}", e))?;
    writer
        .flush()
        .map_err(|e| format_err!("Could not save data: {}", e))?;
    Ok(())
}

/// Gets entry based on user_id
fn get_entry_by_user_id(user_id: &String) -> Result<Entry, Error> {
    // Open the CSV file with a reader
    let file = std::fs::File::open(API_KEY_CSV)
        .map_err(|e| format_err!("Failed to open CSV file: {}", e))?;
    let reader = std::io::BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(false) // Assuming CSV file doesn't contain header
        .from_reader(reader);

    // Process CSV records in parallel
    let entries: Vec<_> = csv_reader.records().collect();
    let entry_result = entries.par_iter().find_first(|result| {
        match result {
            Ok(record) => record.get(0) == Some(user_id),
            _ => false,
        }
    });

    // Extract the entry if found
    match entry_result {
        Some(Ok(record)) => {
            let user_id = user_id.clone();
            let hashed_api_key = record.get(1).unwrap().to_string();
            let permission = UserPermission::from_str(record.get(2).unwrap())
                .map_err(|e| format_err!("Failed to parse permission: {}", e))?;
            Ok(Entry {
                user_id,
                hashed_api_key,
                permission,
            })
        }
        _ => Err(format_err!("Entry not found for user ID: {}", user_id)),
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
pub fn validate_credentials(user_id: &String, api_key: &String, permission_needed: Permissions) -> Result<bool, Error> {
    //create_user(&"Administrator".to_string(), UserPermission::Admin);
    let entry = get_entry_by_user_id(user_id)?;

    // Check if api key is a valid api key for user
    if !verify(api_key, &entry.hashed_api_key)? {return Err(format_err!("Invalid"));}

    match permission_needed {
        Permissions::Read => {
            Ok(entry.permission.read_status())
        }
        Permissions::Write => {
            Ok(entry.permission.write_status())
        }
    }
}
