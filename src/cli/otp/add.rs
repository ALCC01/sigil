// Copyright (C) 2018 Alberto Coscia
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use failure::Error;
use gpgme::Context;
use lib::types::{HmacAlgorithm, OtpRecord};
use lib::utils;
use std::path::PathBuf;

pub fn add_record(
    vault_path: &PathBuf,
    key: &str,
    mut ctx: Context,
    record: OtpRecord,
    record_id: String,
) -> Result<(), Error> {
    let mut vault = utils::read_vault(&vault_path, &mut ctx).unwrap();
    vault.add_otp_record(record, record_id)?;
    utils::write_vault(&vault_path, &vault, &mut ctx, &key).unwrap();

    Ok(())
}

/// Adds an OTP record to the specified vault using an interactive dialog
/**
 * Blueprint
 *  1. Get the OTP record kind from the user (allow Hotp and Totp)
 *  2. Get the information necessary to construct a record from the user or from
 *     the args. Trim all strings.
 *      i) Secret: mandatory
 *      ii) Issuer
 *      iii) Algorithm: default to SHA1
 *      iv) Digits: default to 6
 *      v) Period: default to 30s (TOTP only)
 *  3. Construct a `OtpRecord`
 *  4. Get a record ID from the user, bail if not provided
 *  5. `read_vault`, `vault::add_otp_record`, `write_vault`, bail on error
 */
pub fn add_record_interactive(
    vault_path: &PathBuf,
    key: &str,
    mut ctx: Context,
) -> Result<(), Error> {
    tracepoint!();
    println!("We are going to add a one-time password generator to the vault.");
    println!("Once a generator has been added, it will be safely stored and you'll be able to generate tokens at any time.");
    println!();

    // (1)
    let kind = question!(
        |mut s: String| {
            s = s.to_ascii_lowercase();
            if s.is_empty() {
                Ok("totp".to_string())
            } else if !(s == "totp" || s == "hotp") {
                Err(format_err!("Unknown OTP algorithm"))
            } else {
                Ok(s)
            }
        },
        "What kind of token should be generated? ([TOTP]|HOTP) "
    )?;

    // 2.i
    let secret = question!(
        |s: String| if s.is_empty() {
            Err(format_err!("Please provide a non-empty secret"))
        } else {
            Ok(s)
        },
        "What is the base-32 encoded secret? "
    )?;

    // 2.ii
    let issuer = question!(
        |s: String| if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s.to_owned()))
        },
        "What service issued this secret? [None] "
    )?;

    // 2.iii
    let algorithm = question!(
        |s: String| if s.is_empty() {
            Ok(HmacAlgorithm::SHA1)
        } else {
            Ok(s.parse()?)
        },
        "What HMAC algorithm should be used to generate tokens? ([SHA1]|SHA256|SHA512) "
    )?;

    // 2.iv
    let digits = question!(
        |s: String| if s.is_empty() { Ok(6) } else { Ok(s.parse()?) },
        "How many digits long should a token be? [6] "
    )?;

    let record = match &kind[..] {
        "hotp" => {
            // (3)
            OtpRecord::Hotp {
                secret,
                issuer,
                algorithm,
                digits,
            }
        }
        "totp" => {
            // (2.vi)
            let period = question!(
                |s: String| if s.is_empty() {
                    Ok(30u64)
                } else {
                    Ok(s.parse()?)
                },
                "How often, in seconds, should a new token be generated? [30] "
            )?;

            // (3)
            OtpRecord::Totp {
                secret,
                issuer,
                algorithm,
                period,
                digits,
            }
        }
        _ => unreachable!(),
    };

    // (4)
    let record_id = question!(
        |s: String| if s.is_empty() {
            Err(format_err!("Please provide a non-empty generator"))
        } else {
            Ok(s)
        },
        "How should this generator be called? ",
    )?;

    // (5)
    // TODO These unwraps are due to the fact that the errors cannot be made
    // into failure::Error's. Find a workaround
    let mut vault = utils::read_vault(&vault_path, &mut ctx).unwrap();
    vault.add_otp_record(record, record_id)?;
    utils::write_vault(&vault_path, &vault, &mut ctx, &key).unwrap();

    println!();
    println!("This generator has been successfully added to the vault!");
    Ok(())
}
