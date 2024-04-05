use rusqlite::{Connection, Result};


fn create_db(db_name:str) -> Result<()> {
    let conn = Connection::open(db_name)?;
}

fn create_table(db_name:str, sensor:Sensor) {

}

fn insert_data(db_name:str, data:str) {
    
}
