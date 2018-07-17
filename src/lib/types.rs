use lib::error::{OtpError, VaultError};
use lib::otp;
use ring::digest;
use std::clone::Clone;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub passwords: HashMap<String, Record>,
    pub otps: HashMap<String, OtpRecord>,
}

impl Vault {
    pub fn add_record(&mut self, record: Record, record_id: String) -> Result<(), VaultError> {
        tracepoint!();
        match self.passwords.entry(record_id) {
            Entry::Vacant(entry) => {
                entry.insert(record);
                Ok(())
            }
            _ => Err(VaultError::ShouldUpdate),
        }
    }

    pub fn add_otp_record(
        &mut self,
        record: OtpRecord,
        record_id: String,
    ) -> Result<(), VaultError> {
        tracepoint!();
        match self.otps.entry(record_id) {
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
        match self.passwords.entry(record_id) {
            Entry::Occupied(entry) => {
                entry.remove();
                Ok(())
            }
            _ => Err(VaultError::UnknownRecord(r)),
        }
    }

    pub fn remove_otp_record(&mut self, record_id: String) -> Result<(), VaultError> {
        tracepoint!();
        let r = record_id.clone(); // We need ownership if we need to build an error
        match self.otps.entry(record_id) {
            Entry::Occupied(entry) => {
                entry.remove();
                Ok(())
            }
            _ => Err(VaultError::UnknownRecord(r)),
        }
    }

    pub fn get_record(&self, record_id: String) -> Result<&Record, VaultError> {
        tracepoint!();
        if let Some(record) = self.passwords.get(&record_id) {
            Ok(record)
        } else {
            Err(VaultError::UnknownRecord(record_id))
        }
    }

    pub fn get_otp_record(&self, record_id: String) -> Result<&OtpRecord, VaultError> {
        tracepoint!();
        if let Some(record) = self.otps.get(&record_id) {
            Ok(record)
        } else {
            Err(VaultError::UnknownRecord(record_id))
        }
    }

    pub fn display(&self, disclose: bool, depth: usize) -> String {
        let mut buf = String::new();
        tree_add_element(&mut buf, "Passwords", depth);

        self.passwords.iter().for_each(|record| {
            tree_add_element(&mut buf, record.0, depth + 1);
            buf += &record.1.display(disclose, depth + 2);
        });

        tree_add_element(&mut buf, "OTPs", depth);

        self.otps.iter().for_each(|record| {
            tree_add_element(&mut buf, record.0, depth + 1);
            buf += &record.1.display(disclose, depth + 2);
        });

        buf
    }
}

fn tree_add_element(buf: &mut String, item: &str, depth: usize) {
    let prefix = "│  ".repeat(depth);
    let junction = "├─ ";

    buf.push_str(&format!(
        "{prefix}{junction}{item}\n",
        prefix = prefix,
        junction = junction,
        item = item
    ));
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub home: Option<String>,
}

impl Record {
    pub fn new(
        password: String,
        username: Option<String>,
        email: Option<String>,
        home: Option<String>,
    ) -> Record {
        Record {
            password,
            username,
            email,
            home,
        }
    }

    pub fn display(&self, disclose: bool, depth: usize) -> String {
        let mut buf = String::new();

        if self.home.is_some() {
            tree_add_element(
                &mut buf,
                &format!("Home: {}", self.home.clone().unwrap()),
                depth,
            );
        }
        if self.username.is_some() {
            tree_add_element(
                &mut buf,
                &format!("Username: {}", self.username.clone().unwrap()),
                depth,
            );
        }
        if self.email.is_some() {
            tree_add_element(
                &mut buf,
                &format!("Email: {}", self.email.clone().unwrap()),
                depth,
            );
        }
        if disclose {
            tree_add_element(
                &mut buf,
                &format!("Password: {}", self.password.clone()),
                depth,
            );
        }

        buf
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum OtpRecord {
    Hotp {
        secret: String,
        issuer: Option<String>,
        algorithm: HmacAlgorithm,
        digits: u32,
    },
    Totp {
        secret: String,
        issuer: Option<String>,
        algorithm: HmacAlgorithm,
        period: u64,
        digits: u32,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HmacAlgorithm {
    SHA1,
    SHA256,
    SHA512,
}

impl FromStr for HmacAlgorithm {
    // TODO More informative error
    type Err = OtpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_ascii_uppercase()[..] {
            "SHA1" => Ok(HmacAlgorithm::SHA1),
            "SHA256" => Ok(HmacAlgorithm::SHA256),
            "SHA512" => Ok(HmacAlgorithm::SHA512),
            _ => Err(OtpError::UnknownHmacAlgorithm),
        }
    }
}

impl HmacAlgorithm {
    pub fn to_algorithm(&self) -> otp::Algorithm {
        match self {
            HmacAlgorithm::SHA1 => &digest::SHA1,
            HmacAlgorithm::SHA256 => &digest::SHA256,
            HmacAlgorithm::SHA512 => &digest::SHA512,
        }
    }
}

impl OtpRecord {
    pub fn new_totp(
        secret: String,
        issuer: Option<String>,
        algorithm: HmacAlgorithm,
        digits: u32,
        period: u64,
    ) -> OtpRecord {
        OtpRecord::Totp {
            secret,
            issuer,
            algorithm,
            digits,
            period,
        }
    }
    pub fn new_hotp(
        secret: String,
        issuer: Option<String>,
        algorithm: HmacAlgorithm,
        digits: u32,
    ) -> OtpRecord {
        OtpRecord::Hotp {
            secret,
            issuer,
            algorithm,
            digits,
        }
    }

    /// Generate a token for this record. `counter` is required for Otp::Hotp
    /// and ignored by Otp::Totp
    pub fn generate_token(&self, counter: Option<u64>) -> Result<String, OtpError> {
        match self {
            OtpRecord::Totp {
                secret,
                algorithm,
                period,
                digits,
                ..
            } => {
                let r = otp::totp(0, *period, secret, *digits, &algorithm);
                // RFC 4226 Requires 6-digit values and suggests 7 and 8-digit
                // values, so we 0-pad shorter numbers accordingly
                Ok(match digits {
                    7 => format!("{:07}", r),
                    8 => format!("{:08}", r),
                    _ => format!("{:06}", r),
                })
            }
            OtpRecord::Hotp {
                secret,
                algorithm,
                digits,
                ..
            } => {
                let counter = counter.ok_or(OtpError::NoCounterProvided)?;
                let r = otp::hotp(secret, counter, *digits, &algorithm);

                // RFC 4226 Requires 6-digit values and suggests 7 and 8-digit
                // values, so we 0-pad shorter numbers accordingly
                Ok(match digits {
                    7 => format!("{:07}", r),
                    8 => format!("{:08}", r),
                    _ => format!("{:06}", r),
                })
            }
        }
    }

    pub fn display(&self, disclose: bool, depth: usize) -> String {
        let mut buf = String::new();
        match self {
            OtpRecord::Totp {
                secret,
                algorithm,
                period,
                digits,
                issuer,
            } => {
                tree_add_element(&mut buf, "Type: TOTP", depth);
                if issuer.is_some() {
                    tree_add_element(
                        &mut buf,
                        &format!("Issuer: {}", issuer.clone().unwrap()),
                        depth,
                    );
                }
                tree_add_element(&mut buf, &format!("Algorithm: {:?}", algorithm), depth);
                tree_add_element(&mut buf, &format!("Period: {}s", period), depth);
                tree_add_element(&mut buf, &format!("Digits: {}", digits), depth);
                if disclose {
                    tree_add_element(&mut buf, &format!("Secret: {}", secret), depth);
                }
            }
            OtpRecord::Hotp {
                secret,
                algorithm,
                digits,
                issuer,
            } => {
                tree_add_element(&mut buf, "Type: HOTP", depth);
                if issuer.is_some() {
                    tree_add_element(
                        &mut buf,
                        &format!("Issuer: {}", issuer.clone().unwrap()),
                        depth,
                    );
                }
                tree_add_element(&mut buf, &format!("Algorithm: {:?}", algorithm), depth);
                tree_add_element(&mut buf, &format!("Digits: {}", digits), depth);
                if disclose {
                    tree_add_element(&mut buf, &format!("Secret: {}", secret), depth);
                }
            }
        };

        buf
    }
}
