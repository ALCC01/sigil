#[derive(Debug, Fail)]
pub enum VaultError {
    #[fail(display = "Password hould be updated, not added")]
    ShouldUpdate,
    #[fail(display = "Failed to find record {}", 0)]
    UnknownRecord(String),
}

#[derive(Debug, Fail)]
#[fail(display = "No GPG key was passed either as an argument or as an environment variable")]
pub struct NoKeyError();

#[derive(Debug, Fail)]
#[fail(display = "A mandatory argument was not provided")]
pub struct MandatoryArgumentAbsentError();

#[derive(Debug, Fail)]
#[fail(display = "Failed to find field {} on record", 0)]
pub struct NoSuchField(pub String);
