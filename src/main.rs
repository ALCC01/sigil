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
extern crate rand;
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

use cli::args::{match_args, Sigil};
use structopt::StructOpt;

fn main() {
    env_logger::init();
    tracepoint!();
    // Parse CLI arguments
    let sigil = Sigil::from_args();
    // Match them with a subcommand and run it
    let res = match_args(sigil);
    // Sort of pretty print any error
    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }
}
