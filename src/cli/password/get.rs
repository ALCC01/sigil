use failure::Error;
use gpgme::Context;
use lib::utils;
use std::path::PathBuf;

/// Returns a password from a record
/**
 * Blueprint
 *  1. `read_vault`, `vault.get_record`, bail on error
 *  2. Return the `password` field
 */
pub fn get_password(
    vault_path: &PathBuf,
    mut ctx: Context,
    record_id: String,
) -> Result<(), Error> {
    tracepoint!();

    // (1)
    let vault = utils::read_vault(&vault_path, &mut ctx).unwrap();
    let record = vault.get_record(record_id)?;

    // (2)
    println!("{}", record.password);

    Ok(())
}
