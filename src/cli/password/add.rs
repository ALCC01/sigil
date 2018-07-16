use failure::{Error, Fail};
use gpgme::Context;
use lib::types::Record;
use lib::{error, utils};
use std::path::PathBuf;

/// Adds a password record to the specified vault
/**
 * Blueprint
 *  1. Get the information necessary to construct a record from the user or from
 *     the args. Bail if mandatory info is not provided. Trim all strings. Trim
 *     only newlines for `password`.
 *      a) Service name: mandatory
 *      b) Service URL
 *      c) Account username
 *      d) Account email
 *      e) Account password: mandatory
 *  2. Construct a `Record`
 *  3. Get a record ID from the user, bail if not provided
 *  4. `read_vault`, `Vault::add_record`, `write_vault`, bail on error
 */
// TODO Compact question boilerplate
pub fn add_record(vault_path: &PathBuf, key: &str, mut ctx: Context) -> Result<(), Error> {
    tracepoint!();

    // (1.a)
    let service = question!("What service is this password for? ")?;
    let service = service.trim();
    if service.is_empty() {
        Err(error::MandatoryArgumentAbsentError().context("A service name must be provided"))?
    }

    // (1.b)
    let home = question!("What's the home URL of this service? [None] ")?;
    let home = home.trim();
    let home = if home.is_empty() {
        None
    } else {
        Some(home.to_owned())
    };

    // (1.c)
    let username = question!("What is the username associated with this password? [None] ")?;
    let username = username.trim();
    let username = if username.is_empty() {
        None
    } else {
        Some(username.to_owned())
    };

    // (1.d)
    let email = question!("What is the email associated with this password? [None] ")?;
    let email = email.trim();
    let email = if email.is_empty() {
        None
    } else {
        Some(email.to_owned())
    };

    // (1.e)
    let password = question!("What is the password? ")?;
    let password = password.trim_matches(|n| n == '\n' || n == '\r');
    if password.is_empty() {
        Err(error::MandatoryArgumentAbsentError().context("A password must be provided"))?
    }

    // (2)
    let record = Record {
        username,
        home,
        email,
        password: Some(password.to_owned()),
    };

    // (3)
    let record_id_default = record_id(&record, &service.to_owned());
    let record_id = question!(
        "What should this record be called? [{}] ",
        record_id_default
    )?;
    let mut record_id = record_id.trim().to_owned();
    if record_id.is_empty() {
        record_id = record_id_default;
    }

    // (4)
    // TODO These unwraps are due to the fact that the errors cannot be made
    // into failure::Error's. Find a workaround
    let mut vault = utils::read_vault(&vault_path, &mut ctx).unwrap();
    vault.add_record(record, record_id)?;
    utils::write_vault(&vault_path, &vault, &mut ctx, &key).unwrap();

    Ok(())
}

/// Creates a record id from a `Record`, such as "username:service"
fn record_id(record: &Record, service: &str) -> String {
    match record.username.clone() {
        Some(username) => format!("{}:{}", username, service.to_owned()),
        None => service.to_string(),
    }
}
