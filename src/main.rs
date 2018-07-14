#![feature(attr_literals)]
#![feature(extern_prelude)]
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

/// A macro that expands to a `trace!` with the file name and line
macro_rules! tracepoint {
    () => {{
        trace!("Reached tracepoint at {}:{}", file!(), line!())
    }};
}

/// A macro for asking a question to the user
macro_rules! question {
    ($($arg:tt)*) => {
        {
            use std::io;
            use std::io::Write;
            let mut temp_buf = String::new();
            print!($($arg)*);
            io::stdout().flush().unwrap(); // TODO Can we recover from this?
            io::stdin().read_line(&mut temp_buf).and(Ok(temp_buf))
        }
    };
}

use std::path::PathBuf;
use structopt::StructOpt;

mod cli;
mod lib;

#[derive(Debug, StructOpt)]
#[structopt(name = "sigil", about = "GPG-backed password manager")]
enum Sigil {
    #[structopt(name = "touch")]
    /// Initialize an empty vault file
    Touch {
        #[structopt(parse(from_os_str))]
        /// Path to the vault
        vault: PathBuf,
        #[structopt(short = "K", long = "key")]
        /// The GPG key
        key: Option<String>,
        #[structopt(short = "f", long = "force", takes_value = false)]
        /// Overwrite an existing file
        force: bool,
    },
    #[structopt(name = "ls")]
    /// List all records in a vault
    List {
        #[structopt(parse(from_os_str))]
        /// Path to the vault
        vault: PathBuf,
        #[structopt(long = "disclose", takes_value = false)]
        /// Disclose secrets
        disclose: bool,
    },
    #[structopt(name = "add")]
    /// Add a password to a vault
    Add {
        #[structopt(short = "V", long = "vault", parse(from_os_str))]
        /// Path to the vault
        vault: PathBuf,
        #[structopt(short = "K", long = "key")]
        /// The GPG key
        key: Option<String>,
    },
    #[structopt(name = "rm")]
    /// Remove a record from a vault
    Remove {
        #[structopt(short = "V", long = "vault", parse(from_os_str))]
        /// Path to the vault
        vault: PathBuf,
        #[structopt(short = "K", long = "key")]
        /// The GPG key
        key: Option<String>,
        /// Record name
        record: String,
    },
    #[structopt(name = "get")]
    /// Get a password from a vault
    GetPassword {
        #[structopt(short = "V", long = "vault", parse(from_os_str))]
        /// Path to the vault
        vault: PathBuf,
        #[structopt()]
        /// Record name
        record: String,
    },
    #[structopt(name = "otp")]
    /// Operate on OTP records in a vault
    Otp {
        #[structopt(short = "V", long = "vault", parse(from_os_str))]
        /// Path to the vault
        vault: PathBuf,
        #[structopt(short = "K", long = "key")]
        /// The GPG key to use for encryption
        key: Option<String>,
        #[structopt(subcommand)]
        cmd: OtpCommand,
    },
}

#[derive(Debug, StructOpt)]
enum OtpCommand {
    #[structopt(name = "add")]
    /// Add an OTP generator to a vault
    Add,
    #[structopt(name = "rm")]
    /// Remove an OTP generator
    Remove {
        #[structopt()]
        /// Record name
        record: String,
    },
    #[structopt(name = "token")]
    /// Generate an OTP token
    GetToken {
        #[structopt()]
        /// Record name
        record: String,
        /// Counter for HOTP, ignored for TOTP
        counter: Option<u64>,
    },
}

fn main() {
    env_logger::init();
    tracepoint!();
    let sigil = Sigil::from_args();

    let res = match sigil {
        Sigil::Touch { vault, key, force } => cli::touch::touch_vault(&vault, key, force),
        Sigil::List { vault, disclose } => cli::list::list_vault(&vault, disclose),
        Sigil::Add { vault, key } => cli::add::add_record(&vault, key),
        Sigil::Remove { vault, key, record } => cli::remove::remove_record(&vault, key, record),
        Sigil::GetPassword { vault, record } => cli::get::get_password(&vault, record),
        Sigil::Otp { vault, key, cmd } => match cmd {
            OtpCommand::Add => cli::otp::add::add_record(&vault, key),
            OtpCommand::GetToken { record, counter } => {
                cli::otp::token::get_token(&vault, record, counter)
            }
            OtpCommand::Remove { record } => cli::otp::remove::remove_record(&vault, key, record),
        },
    };
    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }
}
