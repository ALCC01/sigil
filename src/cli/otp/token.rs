// Copyright (C) 2018 Alberto Coscia
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use failure::Error;
use gpgme::Context;
use lib::types::OtpRecord;
use lib::utils;
use std::path::PathBuf;

/// Generates an OTP token
/**
 * Blueprint
 *  1. `read_vault`, `vault::get_otp_record`, bail on error
 *  2. Generate a token, bail on error
 */
pub fn get_token(
    vault_path: &PathBuf,
    mut ctx: Context,
    record_id: &str,
    counter: Option<u64>,
) -> Result<(), Error> {
    tracepoint!();

    // (1)
    let vault = utils::read_vault(&vault_path, &mut ctx).unwrap();
    let record = vault.get_otp_record(&record_id)?;

    // (2)
    let (token, time) = record.generate_token(counter)?;
    println!("Your token is {}", token);
    if let OtpRecord::Totp { .. } = record {
        println!("This token is valid for the next {} seconds", time)
    }

    Ok(())
}
