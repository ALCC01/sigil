use gpgme::{Context, Key, Protocol};
use lib::types::Vault;
use std::error::Error;
use std::fs::{write, File};
use std::path::PathBuf;

/// Creates a GPGME context using the OpenPgp protocol and armor by default
pub fn create_context() -> Result<Context, Box<Error>> {
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;
    ctx.set_armor(true);

    Ok(ctx)
}

/// Decrypts a file
pub fn unlock_file(path: &PathBuf, ctx: &mut Context) -> Result<String, Box<Error>> {
    let mut input = File::open(path)?;

    let mut output: Vec<u8> = Vec::new();
    ctx.decrypt(&mut input, &mut output)?;
    let output = String::from_utf8_lossy(&output).into_owned();

    Ok(output)
}

/// Parses an encrypted vault to `Vault`
pub fn read_vault(path: &PathBuf, ctx: &mut Context) -> Result<Vault, Box<Error>> {
    let string = unlock_file(&path, ctx)?;
    let vault: Vault = serde_json::from_str(&string)?;

    Ok(vault)
}

/// Serializes a `Vault` to an encrypted JSON file
pub fn write_vault(
    path: &PathBuf,
    vault: &Vault,
    ctx: &mut Context,
    key: &str,
) -> Result<(), Box<Error>> {
    let mut input: Vec<u8> = Vec::from(serde_json::to_string_pretty(&vault)?);

    let keys = vec![key];
    let keys: Vec<Key> = ctx
        .find_keys(keys)?
        .filter_map(|x| x.ok())
        .filter(|k| k.can_encrypt())
        .collect();

    let mut output: Vec<u8> = Vec::new();
    ctx.encrypt(&keys, &mut input, &mut output)?;
    let output = String::from_utf8_lossy(&output).into_owned();

    write(path, output)?;
    Ok(())
}
