use crate::{bmi::Bmi, errors::DatabaseError};
use std::io::Write;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DatabaseEntry {
    bmi: crate::bmi::Bmi,
    timestamp: time::PrimitiveDateTime,
}

impl DatabaseEntry {
    pub fn new(bmi: crate::bmi::Bmi) -> Result<Self, DatabaseError> {
        let timestamp = {
            let now = time::OffsetDateTime::now_utc();
            let date = now.date();
            let time = now.time();
            time::PrimitiveDateTime::new(date, time)
        };
        println!("bmi: {}, timestamp: {}", &bmi.value(), &timestamp);
        Ok(Self { bmi, timestamp })
    }

    pub fn timestamp(&self) -> time::PrimitiveDateTime {
        self.timestamp
    }

    pub fn bmi(&self) -> &Bmi {
        &self.bmi
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Database(Vec<DatabaseEntry>);

impl Database {
    pub fn load() -> Result<Self, DatabaseError> {
        let database_str = std::fs::read_to_string("database.json")?;
        let database = serde_json::from_str(&database_str).map_err(DatabaseError::from)?;
        Ok(database)
    }

    pub fn add_entry(&mut self, entry: DatabaseEntry) {
        self.0.push(entry);
    }

    pub fn store(&self) -> Result<(), DatabaseError> {
        let bytes = serde_json::to_vec_pretty(&self)?;

        std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open("database.json")
            .map_err(DatabaseError::from)
            .and_then(|mut file| file.write_all(&bytes).map_err(DatabaseError::from))
    }

    pub fn print(&self) {
        for entry in &self.0 {
            println!("{}: {}", entry.timestamp, entry.bmi.value());
        }
    }

    pub fn entry_iter(&self) -> impl Iterator<Item = &DatabaseEntry> {
        self.0.iter()
    }
}
