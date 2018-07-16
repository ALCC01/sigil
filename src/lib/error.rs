#[derive(Debug, Fail)]
pub enum VaultError {
    #[fail(display = "Record should be updated, not added")]
    ShouldUpdate,
    #[fail(display = "Failed to find record {}", 0)]
    UnknownRecord(String),
    #[fail(display = "Vault path already exists")]
    Overwriting,
    #[fail(display = "Vault path is a directory")]
    VaultIsADirectory,
    #[fail(
        display = "No vault path was provided either as an argument or as an environment variable"
    )]
    NoVault,
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

#[derive(Debug, Fail)]
#[fail(display = "Failed to create a GPG cryptographic context")]
pub struct GgpContextCreationFailed();

#[derive(Debug, Fail)]
pub enum OtpError {
    #[fail(display = "No counter was provided")]
    NoCounterProvided,
    #[fail(display = "Unknown OTP algorithm")]
    UnknownAlgorithm,
    #[fail(display = "Unknown HMAC algorithm")]
    UnknownHmacAlgorithm,
}
