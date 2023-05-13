use std::io::Write;
use crate::errors::DatabaseError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DatabaseEntry {
    bmi: crate::bmi::Bmi,
    timestamp: time::PrimitiveDateTime,
}

impl DatabaseEntry {
    pub fn new(bmi: crate::bmi::Bmi) -> Result<Self, DatabaseError> {
        let timestamp = {
            let now = time::OffsetDateTime::now_local().map_err(DatabaseError::from)?;
            let date = now.date();
            let time = now.time();
            time::PrimitiveDateTime::new(date, time)
        };

        Ok(Self { bmi, timestamp })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Database(Vec<DatabaseEntry>);

impl Database {
    pub fn load() -> Result<Self, DatabaseError> {
        let database_str = std::fs::read_to_string("bmi_file.txt")?;
        let database = serde_json::from_str(&database_str).map_err(DatabaseError::from)?;
        Ok(database)
    }

    pub fn add_entry(&mut self, entry: DatabaseEntry) {
        self.0.push(entry);
    }

    pub fn store(self) -> Result<(), DatabaseError> {
        let bytes = serde_json::to_vec_pretty(&self)?;

        std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open("bmi_file.txt")
            .map_err(DatabaseError::from)
            .and_then(|mut file| file.write_all(&bytes).map_err(DatabaseError::from))
    }

    pub fn print(&self) {
        for entry in &self.0 {
            println!("{}: {}", entry.timestamp, entry.bmi.value());
        }
    }
}