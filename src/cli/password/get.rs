use failure::Error;
use gpgme::Context;
use lib::error;
use lib::utils;
use std::path::PathBuf;

/// Returns a password from a record
/**
 * Blueprint
 *  1. `read_vault`, `vault.get_record`, bail on error
 *  2. Return the `password` field or bail if it is not defined
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
    match record.password {
        Some(ref password) => {
            println!("{}", password);
            Ok(())
        }
        None => Err(error::NoSuchField("password".to_string()))?,
    }
}
