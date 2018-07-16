use failure::Error;
use lib::utils;
use std::path::PathBuf;

/// Lists all records in a vault
/**
 * Blueprint
 *  1. `read_vault`, bail on error
 *  2. `vault.display`
 */
pub fn list_vault(vault_path: &PathBuf, disclose: bool) -> Result<(), Error> {
    tracepoint!();
    // (1)
    // Acquire a GPGME context
    // TODO Can we handle these failures more nicely?
    let mut ctx = utils::create_context().unwrap();
    let vault = utils::read_vault(&vault_path, &mut ctx).unwrap();

    // (2)
    println!("{}", vault_path.display());
    print!("{}", vault.display(disclose, 0));

    Ok(())
}
