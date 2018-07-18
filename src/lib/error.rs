#[derive(Debug, Fail)]
pub enum VaultError {
    #[fail(display = "Record should be updated, not added")]
    ShouldUpdate,
    #[fail(display = "Failed to find a matching record")]
    UnknownRecord,
}

#[derive(Debug, Fail)]
pub enum OtpError {
    #[fail(display = "No counter was provided")]
    NoCounterProvided,
    #[fail(display = "Unknown HMAC algorithm")]
    UnknownHmacAlgorithm,
}
