use failure::Error;
use lib::utils;
use std::path::PathBuf;

/// Lists all records in a vault
/**
 * Blueprint
 *  1. `read_vault`, bail on error
 *  2. `vault.display`
 */
pub fn list_vault(vault: &PathBuf, disclose: bool) -> Result<(), Error> {
    tracepoint!();
    let vault_path = vault.clone(); // `vault` will be shadowed later on

    // (1)
    // Acquire a GPGME context
    // TODO Can we handle these failures more nicely?
    let mut ctx = utils::create_context().unwrap();
    let vault = utils::read_vault(&vault, &mut ctx).unwrap();

    // (2)
    tracepoint!();
    println!("{}", vault_path.display());
    println!("{}", vault.display(disclose, 0, true));

    Ok(())
}
