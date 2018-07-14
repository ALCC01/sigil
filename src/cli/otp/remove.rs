use failure::Error;
use lib::error;
use lib::utils;
use std::path::PathBuf;

/**
 * Blueprint
 *  1. Get the enc key from the user or the environment, otherwise bail
 *  2. `read_vault`, `vault.remove_otp_record`, `write_vault`, bail on error
 */
pub fn remove_record(vault: &PathBuf, key: Option<String>, record_id: String) -> Result<(), Error> {
    tracepoint!();
    let vault_path = vault.clone(); // `vault` will be shadowed later on

    // (1)
    // Acquire a GPGME context
    // TODO Can we handle this failure more nicely?
    let mut ctx = utils::create_context().unwrap();
    // A key can either be provided as an argument or an environment var
    let key = key
        .or_else(|| std::env::var_os("GPGKEY").map(|n| n.to_string_lossy().to_string()))
        .ok_or(error::NoKeyError())?
        .to_owned();

    // (2)
    // TODO These unwraps are due to the fact that the errors cannot be made
    // into failure::Error's. Find a workaround
    tracepoint!();
    let mut vault = utils::read_vault(&vault, &mut ctx).unwrap();
    vault.remove_otp_record(record_id)?;
    utils::write_vault(&vault_path, &vault, &mut ctx, &key).unwrap();
    Ok(())
}
