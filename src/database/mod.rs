
/// Utility modules to offer quick functionality
mod error;
mod util;

/// Three main databases
pub mod catalog_database;
pub mod device_database;
pub mod api_keys_sqlite;

/// Initializes all databases
pub fn init() {
    catalog_database::initialize_database();
    api_keys_sqlite::init_database();
}
