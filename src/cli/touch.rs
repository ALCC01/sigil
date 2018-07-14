use failure::Error;
use lib::error;
use lib::types::Vault;
use lib::utils;
use std::collections::HashMap;
use std::path::PathBuf;

/// Creates an empty vault file
/**
 * Blueprint
 *  1. Get the enc key from the user or the environment, otherwise bail
 *  2. Check that we're not writing on an existing file, bail if true and not
 *     --force'ing. Check that existing file is not a directory, bail if true.
 *  3. Construct an empty `Vault`
 *  4. `write_vault`, bail on error
 */
pub fn touch_vault(vault: &PathBuf, key: Option<String>, force: bool) -> Result<(), Error> {
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
    // Check if file exists
    if vault_path.exists() && !force {
        Err(error::VaultError::Overwriting)?
    }
    // Check if file is a directory
    if vault_path.is_dir() {
        Err(error::VaultError::VaultIsADirectory)?
    }

    // (3)
    let vault = Vault {
        passwords: HashMap::new(),
        otps: HashMap::new(),
    };

    // (4)
    // TODO Can we handle this failure more nicely?
    utils::write_vault(&vault_path, &vault, &mut ctx, &key).unwrap();
    Ok(())
}
