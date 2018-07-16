use failure::Error;
use gpgme::Context;
use lib::utils;
use std::path::PathBuf;

/// Generates an OTP token
/**
 * Blueprint
 *  1. `read_vault`, `vault::get_otp_record`, bail on error
 *  2. Generate a token, bail on error
 */
pub fn get_token(
    vault_path: &PathBuf,
    mut ctx: Context,
    record_id: String,
    counter: Option<u64>,
) -> Result<(), Error> {
    tracepoint!();

    // (1)
    let vault = utils::read_vault(&vault_path, &mut ctx).unwrap();
    let record = vault.get_otp_record(record_id)?;

    // (2)
    tracepoint!();
    println!("{}", record.generate_token(counter)?);

    Ok(())
}
