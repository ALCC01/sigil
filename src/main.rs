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
extern crate gpgme;
extern crate serde;
extern crate serde_json;

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

use failure::Error;
use std::path::PathBuf;
use structopt::StructOpt;

mod cli;
mod lib;

#[derive(Debug, StructOpt)]
#[structopt(name = "sigil", about = "GPG-backed password manager")]
enum Sigil {
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
}

fn main() -> Result<(), Error> {
    env_logger::init();
    tracepoint!();
    let sigil = Sigil::from_args();

    match sigil {
        Sigil::Add { vault, key } => cli::add::add_record(&vault, key),
        Sigil::GetPassword { vault, record } => cli::get::get_password(&vault, record),
    }
}
