use cli;
use failure::Error;
use lib::{error, utils};
use std::env;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "sigil", about = "GPG-backed password manager")]
pub struct Sigil {
    #[structopt(short = "V", long = "vault", parse(from_os_str))]
    /// Path to the vault. Required if not set by the SIGIL_VAULT environment
    /// variable
    pub vault: Option<PathBuf>,
    #[structopt(short = "K", long = "key")]
    /// The GPG key to use for encryption. Required for operations that will
    /// write on a vault. Defaults to the GPGKEY environment variable
    pub key: Option<String>,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "password")]
    /// Operate on password records in a vault
    Password {
        #[structopt(subcommand)]
        cmd: PasswordCommand,
    },
    #[structopt(name = "otp")]
    /// Operate on OTP records in a vault
    Otp {
        #[structopt(subcommand)]
        cmd: OtpCommand,
    },
    #[structopt(name = "touch")]
    /// Initialize an empty vault file
    Touch {
        #[structopt(short = "f", long = "force", takes_value = false)]
        /// Overwrite an existing file
        force: bool,
    },
    #[structopt(name = "ls")]
    /// List all records in a vault
    List {
        #[structopt(long = "disclose", takes_value = false)]
        /// Disclose secrets
        disclose: bool,
    },
}

#[derive(Debug, StructOpt)]
pub enum OtpCommand {
    #[structopt(name = "add")]
    /// Add an OTP generator to a vault
    Add,
    #[structopt(name = "import")]
    /// Import an OTP generator to a vault using an `otpauth://` URI
    ImportUrl {
        #[structopt()]
        url: String,
    },
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

#[derive(Debug, StructOpt)]
pub enum PasswordCommand {
    #[structopt(name = "add")]
    /// Add a password to a vault
    Add,
    #[structopt(name = "rm")]
    /// Remove a record from a vault
    Remove {
        /// Record name
        record: String,
    },
    #[structopt(name = "get")]
    /// Get a password from a vault
    GetPassword {
        #[structopt()]
        /// Record name
        record: String,
    },
    #[structopt(name = "generate")]
    /// Generate a random password
    Generate {
        #[structopt()]
        /// Password length
        chars: usize,
    },
}

pub fn match_args(sigil: Sigil) -> Result<(), Error> {
    // Try to fetch sigil key and vault from the environment
    // Not all commands will need these
    let key = sigil
        .key
        .or_else(|| env::var_os("GPGKEY").map(|n| n.to_string_lossy().to_string()))
        .ok_or(error::NoKeyError());
    let vault = sigil
        .vault
        .or_else(|| env::var_os("SIGIL_VAULT").map(PathBuf::from))
        .ok_or(error::VaultError::NoVault);
    // Not all commands will need a context
    let ctx = utils::create_context().map_err(|_| error::GgpContextCreationFailed());

    match sigil.cmd {
        Command::Touch { force } => cli::touch::touch_vault(&vault?, &key?, force),
        Command::List { disclose } => cli::list::list_vault(&vault?, disclose),
        Command::Password { cmd } => match cmd {
            PasswordCommand::Add => cli::password::add_record(&vault?, &key?, ctx?),
            PasswordCommand::Remove { record } => {
                cli::password::remove_record(&vault?, &key?, ctx?, record)
            }
            PasswordCommand::GetPassword { record } => {
                cli::password::get_password(&vault?, ctx?, record)
            }
            PasswordCommand::Generate { chars } => cli::password::generate_password(chars),
        },
        Command::Otp { cmd } => match cmd {
            OtpCommand::Add => cli::otp::add_record(&vault?, &key?, ctx?),
            OtpCommand::ImportUrl { url } => cli::otp::import_url(&vault?, &key?, ctx?, &url),
            OtpCommand::GetToken { record, counter } => {
                cli::otp::get_token(&vault?, ctx?, record, counter)
            }
            OtpCommand::Remove { record } => cli::otp::remove_record(&vault?, &key?, ctx?, record),
        },
    }
}
