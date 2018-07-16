use failure::Error;
use lib::error;
use lib::types::Vault;
use lib::utils;
use std::collections::HashMap;
use std::path::PathBuf;

/// Creates an empty vault file
/**
 * Blueprint
 *  1. Check that we're not writing on an existing file, bail if true and not
 *     --force'ing. Check that existing file is not a directory, bail if true.
 *  2. Construct an empty `Vault`
 *  3. `write_vault`, bail on error
 */
pub fn touch_vault(vault_path: &PathBuf, key: &str, force: bool) -> Result<(), Error> {
    tracepoint!();
    // Acquire a GPGME context
    // TODO Can we handle this failure more nicely?
    let mut ctx = utils::create_context().unwrap();

    // (1)
    // Check if file exists
    if vault_path.exists() && !force {
        Err(error::VaultError::Overwriting)?
    }
    // Check if file is a directory
    if vault_path.is_dir() {
        Err(error::VaultError::VaultIsADirectory)?
    }

    // (2)
    let vault = Vault {
        passwords: HashMap::new(),
        otps: HashMap::new(),
    };

    // (3)
    // TODO Can we handle this failure more nicely?
    utils::write_vault(&vault_path, &vault, &mut ctx, &key).unwrap();

    Ok(())
}
