use chrono::{Local, DateTime};
pub use rusqlite::{Connection, Result};


fn get_current_time() -> DateTime<Local>{
    Local::now()
}
pub struct Table {
    pub table_name:String,
    pub value_type:String
}

impl Table {
    pub fn new(table_name:String, value_type:String) -> Self {
        Table {
            table_name,
            value_type,
        }
    }

    pub fn create_table(&self, conn:&Connection) {
        let command = format!(
        "CREATE TABLE {} (
            id    INTEGER PRIMARY KEY,
            data  {} NOT NULL,
            timestamp TEXT NOT NULL
        )", self.table_name, self.value_type);
    }
    
    pub fn insert(&self, mut conn:&Connection, data:String) {
        let command = format!("INSERT INTO {} (data, timestamp) VALUES (?1, ?2)", self.table_name);
        conn.execute(command.as_str(), (data, get_current_time().to_string()));
    }
}