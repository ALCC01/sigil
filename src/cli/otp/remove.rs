// Copyright (C) 2018 Alberto Coscia
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use failure::Error;
use gpgme::Context;
use lib::utils;
use std::path::PathBuf;

/// Removes and OTP record from a vault
/**
 * Blueprint
 *  1. `read_vault`, `vault.remove_otp_record`, `write_vault`, bail on error
 */
pub fn remove_record(
    vault_path: &PathBuf,
    key: &str,
    mut ctx: Context,
    record_id: String,
) -> Result<(), Error> {
    tracepoint!();

    // (1)
    // TODO These unwraps are due to the fact that the errors cannot be made
    // into failure::Error's. Find a workaround
    let mut vault = utils::read_vault(&vault_path, &mut ctx).unwrap();
    vault.remove_otp_record(record_id)?;
    utils::write_vault(&vault_path, &vault, &mut ctx, &key).unwrap();

    Ok(())
}
