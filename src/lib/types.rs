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

    pub fn remove_record(&mut self, record_id: String) -> Result<(), VaultError> {
        tracepoint!();
        let r = record_id.clone(); // We need ownership if we need to build an error
        match self.services.entry(record_id) {
            Entry::Occupied(entry) => {
                entry.remove();
                Ok(())
            }
            _ => Err(VaultError::UnknownRecord(r)),
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

    pub fn display(&self, disclose: bool, depth: usize, last: bool) -> String {
        let mut buff = String::new();
        let prefix = if !last { "│  " } else { "   " }.repeat(depth);

        self.services.iter().enumerate().for_each(|(i, record)| {
            buff += &format!(
                "{}{}─ {}\n",
                prefix,
                if i != self.services.len() - 1 {
                    "├"
                } else {
                    "└"
                },
                record.0
            );
            buff += &record
                .1
                .display(disclose, depth + 1, i == self.services.len() - 1);
        });

        buff
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub home: Option<String>,
}

impl Record {
    pub fn display(&self, disclose: bool, depth: usize, last: bool) -> String {
        let mut buff = String::new();
        let prefix = if !last { "│  " } else { "   " }.repeat(depth);

        if self.home.is_some() {
            buff += &format!(
                "{}├─ {}: {}\n",
                prefix,
                "Home",
                self.home.clone().unwrap()
            );
        }
        if self.username.is_some() {
            buff += &format!(
                "{}├─ {}: {}\n",
                prefix,
                "Username",
                self.username.clone().unwrap()
            );
        }
        if self.email.is_some() {
            buff += &format!(
                "{}├─ {}: {}\n",
                prefix,
                "Email",
                self.email.clone().unwrap()
            );
        }
        if self.password.is_some() && disclose {
            buff += &format!(
                "{}├─ {}: {}\n",
                prefix,
                "Password",
                self.password.clone().unwrap()
            );
        }

        buff
    }
}
