use failure::Error;
use gpgme::Context;
use lib::types::Record;
use lib::utils;
use std::path::PathBuf;

/// Adds the provided password record to the specified vault
/**
 * Blueprint
 *  1. `read_vault`, `vault::add_record`, `write_vault`, bail on error
 */
pub fn add_record(
    vault_path: &PathBuf,
    key: &str,
    mut ctx: Context,
    record: Record,
    record_id: String,
) -> Result<(), Error> {
    tracepoint!();

    // (1)
    // TODO These unwraps are due to the fact that the errors cannot be made
    // into failure::Error's. Find a workaround
    let mut vault = utils::read_vault(&vault_path, &mut ctx).unwrap();
    vault.add_record(record, record_id)?;
    utils::write_vault(&vault_path, &vault, &mut ctx, &key).unwrap();

    Ok(())
}

/// Adds a password record to the specified vault using an interactive dialog
/**
 * Blueprint
 *  1. Get the information necessary to construct a record from the user or from
 *     the args. Trim all strings.
 *      a) Service name: mandatory
 *      b) Service URL
 *      c) Account username
 *      d) Account email
 *      e) Account password: mandatory
 *  2. Construct a `Record`
 *  3. Get a record ID from the user, bail if not provided
 *  4. `add_record`
 */
pub fn add_record_interactive(vault_path: &PathBuf, key: &str, ctx: Context) -> Result<(), Error> {
    tracepoint!();
    println!("We are going to add a password to the vault.");
    println!("Once a password has been added, it will be safely stored and you'll be able to access it at any time.");
    println!();

    // (1.a)
    let service = question!(
        |s: String| if s.is_empty() {
            Err(format_err!("Please provide a service name"))
        } else {
            Ok(s)
        },
        "What service is this password for? "
    )?;

    // (1.b)
    let home = question!(
        |s: String| if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s.to_owned()))
        },
        "What's the home URL of this service? [None] "
    )?;

    // (1.c)
    let username = question!(
        |s: String| if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s.to_owned()))
        },
        "What username do you use to log in with this password? [None] "
    )?;

    // (1.d)
    let email = question!(
        |s: String| if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s.to_owned()))
        },
        "What's the email associated with this password? [None] "
    )?;

    // (1.e)
    let password = question!(
        |s: String| if s.is_empty() {
            Err(format_err!("Please provide a non-empty password"))
        } else {
            Ok(s)
        },
        "What's the password? "
    )?;

    // (2)
    let record = Record {
        username,
        home,
        email,
        password: password.to_owned(),
    };

    // (3)
    let record_id_default = record_id(&record, &service.to_owned());
    let record_id = question!(
        |s: String| if s.is_empty() {
            Ok(record_id_default.clone())
        } else {
            Ok(s)
        },
        "How should this password be called? [{}] ",
        record_id_default.clone()
    )?;

    // (4)
    add_record(&vault_path, &key, ctx, record, record_id)?;

    println!();
    println!("This password has been successfully added to the vault!");
    Ok(())
}

/// Creates a record id from a `Record`, such as "username:service"
fn record_id(record: &Record, service: &str) -> String {
    match record.username.clone() {
        Some(username) => format!("{}:{}", username, service.to_owned()),
        None => service.to_string(),
    }
}
