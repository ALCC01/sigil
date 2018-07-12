use failure::Error;
use lib::error;
use lib::utils;
use std::path::PathBuf;

/// Returns a password from a record
/**
 * Blueprint
 *  1. `read_vault`, `Vault::get_record`, bail on error
 *  2. Return the `password` field or bail if it is not defined
 */
pub fn get_password(vault: &PathBuf, record_id: String) -> Result<(), Error> {
    tracepoint!();

    // (1)
    // Acquire a GPGME context
    // TODO Can we handle these failures more nicely?
    let mut ctx = utils::create_context().unwrap();
    let vault = utils::read_vault(&vault, &mut ctx).unwrap();
    let record = vault.get_record(record_id)?;

    // (2)
    tracepoint!();
    match record.password {
        Some(ref password) => {
            println!("{}", password);
            Ok(())
        }
        None => Err(error::NoSuchField("password".to_string()))?,
    }
}
