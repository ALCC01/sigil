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
}
