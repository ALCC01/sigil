use failure::Error;
use lib::utils;
use std::path::PathBuf;

/// Generates an OTP token
/**
 * Blueprint
 *  1. `read_vault`, `vault::get_otp_record`, bail on error
 *  2. Generate a token, bail on error
 */
pub fn get_token(vault: &PathBuf, record_id: String, counter: Option<u64>) -> Result<(), Error> {
    tracepoint!();

    // (1)
    // Acquire a GPGME context
    // TODO Can we handle these failures more nicely?
    let mut ctx = utils::create_context().unwrap();
    let vault = utils::read_vault(&vault, &mut ctx).unwrap();
    let record = vault.get_otp_record(record_id)?;

    // (2)
    tracepoint!();
    println!("{}", record.generate_token(counter)?);

    Ok(())
}
