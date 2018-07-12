use failure::{Error, Fail};
use lib::types::Record;
use lib::{error, utils};
use std::path::PathBuf;

/// Adds a record to the specified vault
/**
 * Blueprint
 *  1. Get the enc key from the user or the environment, otherwise bail
 *  2. Get the information necessary to construct a record from the user or from
 *     the args. Bail if mandatory info is not provided. Trim all strings. Trim
 *     only newlines for `password`.
 *      a) Service name: mandatory
 *      b) Service URL
 *      c) Account username
 *      d) Account email
 *      e) Account password: mandatory
 *  3. Construct a `Record`
 *  4. Get a record ID from the user, bail if not provided
 *  5. `read_vault`, `Vault::add_record`, `write_vault`, bail on error
 */
// TODO Compact question boilerplate
pub fn add_record(vault: &PathBuf, key: Option<String>) -> Result<(), Error> {
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

    // (2.a)
    let service = question!("What service is this password for? ")?;
    let service = service.trim();
    if service.is_empty() {
        Err(error::MandatoryArgumentAbsentError().context("A service name must be provided"))?
    }

    // (2.b)
    let home = question!("What's the home URL of this service? [None] ")?;
    let home = home.trim();
    let home = if home.is_empty() {
        None
    } else {
        Some(home.to_owned())
    };

    // (2.c)
    let username = question!("What is the username associated with this password? [None] ")?;
    let username = username.trim();
    let username = if username.is_empty() {
        None
    } else {
        Some(username.to_owned())
    };

    // (2.d)
    let email = question!("What is the email associated with this password? [None] ")?;
    let email = email.trim();
    let email = if email.is_empty() {
        None
    } else {
        Some(email.to_owned())
    };

    // (2.e)
    let password = question!("What is the password? ")?;
    let password = password.trim_matches(|n| n == '\n' || n == '\r');
    if password.is_empty() {
        Err(error::MandatoryArgumentAbsentError().context("A password must be provided"))?
    }

    // (3)
    tracepoint!();
    let record = Record {
        username,
        home,
        email,
        password: Some(password.to_owned()),
    };

    // (4)
    tracepoint!();
    let record_id_default = record_id(&record, &service.to_owned());
    let record_id = question!("What should this record be called? [{}] ", record_id_default)?;
    let mut record_id = record_id.trim().to_owned();
    if record_id.is_empty() {
        record_id = record_id_default;
    }

    // (5)
    // TODO These unwraps are due to the fact that the errors cannot be made
    // into failure::Error's. Find a workaround
    tracepoint!();
    let mut vault = utils::read_vault(&vault, &mut ctx).unwrap();
    vault.add_record(record, record_id)?;
    utils::write_vault(&vault_path, &vault, &mut ctx, &key).unwrap();
    Ok(())
}

/// Creates a record id from a `Record`, such as "username@service"
fn record_id(record: &Record, service: &str) -> String {
    match record.username.clone() {
        Some(username) => format!("{}@{}", username, service.to_owned()),
        None => service.to_string(),
    }
}
