use lib::error::VaultError;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub services: HashMap<String, Record>,
}

impl Vault {
    pub fn add_record(&mut self, record: Record, record_id: String) -> Result<(), VaultError> {
        tracepoint!();
        match self.services.entry(record_id) {
            Entry::Vacant(entry) => {
                entry.insert(record);
                Ok(())
            }
            _ => Err(VaultError::ShouldUpdate),
        }
    }

    pub fn get_record(&self, record_id: String) -> Result<&Record, VaultError> {
        tracepoint!();
        if let Some(record) = self.services.get(&record_id) {
            Ok(record)
        } else {
            Err(VaultError::UnknownRecord(record_id))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub home: Option<String>,
}
