// Copyright (C) 2018 Alberto Coscia
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cli;
use failure::Error;
use lib::types::{HmacAlgorithm, OtpRecord, Record};
use lib::utils;
use std::env;
use std::io;
use std::path::PathBuf;
use structopt::clap::{ArgGroup, Shell};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "sigil")]
pub struct Sigil {
    #[structopt(short = "V", long = "vault", parse(from_os_str))]
    /// Path to the vault. Defaults to the SIGIL_VAULT environment variable
    pub vault: Option<PathBuf>,
    #[structopt(short = "K", long = "key")]
    /// The GPG key to use for encryption. Required for operations that will
    /// write on a vault. Defaults to the SIGIL_GPGKEY environment variable
    pub key: Option<String>,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "password")]
    /// Operate on passwords in a vault
    Password {
        #[structopt(subcommand)]
        cmd: PasswordCommand,
    },
    #[structopt(name = "otp")]
    /// Operate on OTP generators in a vault
    Otp {
        #[structopt(subcommand)]
        cmd: OtpCommand,
    },
    #[structopt(name = "touch")]
    /// Initialize an empty vault file
    Touch {
        #[structopt(short = "f", long = "force", raw(takes_value = "false"))]
        /// Overwrite an existing file
        force: bool,
    },
    #[structopt(name = "ls")]
    /// List all secrets in a vault
    List {
        #[structopt(long = "disclose", raw(takes_value = "false"))]
        /// Disclose secrets
        disclose: bool,
    },
    #[structopt(name = "completion")]
    /// Generate a completion script for Sigil
    Completion {
        #[structopt(raw(possible_values = "&Shell::variants()"))]
        /// The shell that will be using the script
        shell: Shell,
    },
}

fn algo_arg_group() -> ArgGroup<'static> {
    ArgGroup::with_name("algo").required(true)
}

#[derive(Debug, StructOpt)]
#[structopt(raw(group = "algo_arg_group()"))]
pub enum OtpCommand {
    #[structopt(name = "add")]
    /// Add an OTP generator to a vault. Interactive mode if no argument is provided
    Add {
        #[structopt(long = "totp", raw(takes_value = "false"), group = "algo")]
        /// Use TOTP as the generation algorithm
        totp: bool,
        #[structopt(long = "hotp", raw(takes_value = "false"), group = "algo")]
        /// Use HOTP as the generation algorithm
        hotp: bool,
        /// A label for this secret
        #[structopt(requires = "secret")]
        name: Option<String>,
        #[structopt(raw(requires_all = "&[\"algo\", \"name\"]"))]
        /// The secret  
        secret: Option<String>,
        #[structopt(requires = "secret", long = "issuer")]
        /// The issuer of this secret
        issuer: Option<String>,
        #[structopt(requires = "secret", long = "hmac")]
        /// The HMAC algorithm to use to generate tokens
        algorithm: Option<HmacAlgorithm>,
        #[structopt(requires = "secret", long = "digits")]
        /// The token length
        digits: Option<u32>,
        #[structopt(requires = "secret", long = "period")]
        /// Token validity in seconds
        period: Option<u64>,
    },
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
        /// Generator name
        name: String,
    },
    #[structopt(name = "token")]
    /// Generate an OTP token
    GetToken {
        #[structopt()]
        /// Generator name
        name: String,
        /// Counter for HOTP, ignored for TOTP
        counter: Option<u64>,
    },
}

#[derive(Debug, StructOpt)]
pub enum PasswordCommand {
    #[structopt(name = "add")]
    /// Add a password to a vault. Interactive mode if no argument is provided
    Add {
        #[structopt(requires = "password")]
        /// A label for this password
        name: Option<String>,
        #[structopt()]
        /// The password
        password: Option<String>,
        #[structopt(short = "u", long = "username", requires = "password")]
        /// The username associated with this password
        username: Option<String>,
        #[structopt(long = "email", requires = "password")]
        /// The email associated with this password
        email: Option<String>,
        #[structopt(long = "home", requires = "password")]
        /// The homepage for this service
        home: Option<String>,
    },
    #[structopt(name = "rm")]
    /// Remove a password from a vault
    Remove {
        /// Password name
        name: String,
    },
    #[structopt(name = "get")]
    /// Get a password from a vault
    GetPassword {
        #[structopt()]
        /// Password name
        name: String,
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
        .or_else(|| env::var_os("SIGIL_GPGKEY").map(|n| n.to_string_lossy().to_string()))
        .ok_or_else(|| {
            format_err!("No GPG key was passed either as an argument (--key) or as an environment variable (SIGIL_GPGKEY)")
        });
    let vault = sigil
        .vault
        .or_else(|| env::var_os("SIGIL_VAULT").map(PathBuf::from))
        .ok_or_else(|| {
            format_err!("No vault path was passed either as an argument (--vault) or as an environment variable (SIGIL_VAULT)")
        });
    // Not all commands will need a context
    let ctx = utils::create_context()
        .map_err(|_| format_err!("Failed to create a GPG cryptographic context"));

    match sigil.cmd {
        Command::Touch { force } => cli::touch::touch_vault(&vault?, &key?, force),
        Command::List { disclose } => cli::list::list_vault(&vault?, disclose),
        Command::Completion { shell } => {
            Sigil::clap().gen_completions_to("sigil", shell, &mut io::stdout());

            Ok(())
        }
        Command::Password { cmd } => match cmd {
            PasswordCommand::Add {
                name,
                password,
                username,
                email,
                home,
            } => {
                if name.is_some() && password.is_some() {
                    // Safe unwraps because we checked them before and they are required args
                    cli::password::add_record(
                        &vault?,
                        &key?,
                        ctx?,
                        Record::new(password.unwrap(), username, email, home),
                        name.unwrap(),
                    )
                } else {
                    cli::password::add_record_interactive(&vault?, &key?, ctx?)
                }
            }
            PasswordCommand::Remove { name } => {
                cli::password::remove_record(&vault?, &key?, ctx?, name)
            }
            PasswordCommand::GetPassword { name } => {
                cli::password::get_password(&vault?, ctx?, &name)
            }
            PasswordCommand::Generate { chars } => cli::password::generate_password(chars),
        },
        Command::Otp { cmd } => match cmd {
            OtpCommand::Add {
                totp,
                hotp,
                issuer,
                name,
                secret,
                algorithm,
                digits,
                period,
            } => {
                if secret.is_some() && name.is_some() {
                    // Safe unwraps because we checked them before and they are required args
                    if totp {
                        cli::otp::add_record(
                            &vault?,
                            &key?,
                            ctx?,
                            OtpRecord::new_totp(
                                secret.unwrap(),
                                issuer,
                                algorithm.unwrap_or(HmacAlgorithm::SHA1),
                                digits.unwrap_or(6),
                                period.unwrap_or(30),
                            ),
                            name.unwrap(),
                        )
                    } else if hotp {
                        cli::otp::add_record(
                            &vault?,
                            &key?,
                            ctx?,
                            OtpRecord::new_hotp(
                                secret.unwrap(),
                                issuer,
                                algorithm.unwrap_or(HmacAlgorithm::SHA1),
                                digits.unwrap_or(6),
                            ),
                            name.unwrap(),
                        )
                    } else {
                        unreachable!()
                    }
                } else {
                    cli::otp::add_record_interactive(&vault?, &key?, ctx?)
                }
            }
            OtpCommand::ImportUrl { url } => cli::otp::import_url(&vault?, &key?, ctx?, &url),
            OtpCommand::GetToken { name, counter } => {
                cli::otp::get_token(&vault?, ctx?, &name, counter)
            }
            OtpCommand::Remove { name } => cli::otp::remove_record(&vault?, &key?, ctx?, name),
        },
    }
}
