#![feature(attr_literals)]
extern crate env_logger; // TODO env_logger may not be a good fit
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate base32;
extern crate gpgme;
extern crate ring;
extern crate serde;
extern crate toml;
extern crate url;

/// A macro that expands to a `trace!` with the file name and line
/// Disabled in releases
macro_rules! tracepoint {
    () => {{
        #[cfg(debug_assertions)]
        trace!("Reached tracepoint at {}:{}", file!(), line!())
    }};
}

mod cli;
mod lib;

use cli::args::{Command, OtpCommand, PasswordCommand, Sigil};
use lib::error;
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    env_logger::init();
    tracepoint!();
    let sigil = Sigil::from_args();

    // The command will decide if it needs a key
    let key = sigil
        .key
        .or_else(|| env::var_os("GPGKEY").map(|n| n.to_string_lossy().to_string()));

    // Can't work without a vault
    let vault = sigil
        .vault
        .or_else(|| env::var_os("SIGIL_VAULT").map(PathBuf::from))
        .ok_or(error::VaultError::NoVault)
        .unwrap();

    let res = match sigil.cmd {
        Command::Touch { force } => cli::touch::touch_vault(&vault, key, force),
        Command::List { disclose } => cli::list::list_vault(&vault, disclose),
        Command::Password { cmd } => match cmd {
            PasswordCommand::Add => cli::password::add_record(&vault, key),
            PasswordCommand::Remove { record } => cli::password::remove_record(&vault, key, record),
            PasswordCommand::GetPassword { record } => cli::password::get_password(&vault, record),
        },
        Command::Otp { cmd } => match cmd {
            OtpCommand::Add => cli::otp::add_record(&vault, key),
            OtpCommand::ImportUrl { url } => cli::otp::import_url(&vault, key, &url),
            OtpCommand::GetToken { record, counter } => {
                cli::otp::get_token(&vault, record, counter)
            }
            OtpCommand::Remove { record } => cli::otp::remove_record(&vault, key, record),
        },
    };
    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }
}
