// Copyright (C) 2018 Alberto Coscia
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use failure::Error;
use gpgme::Context;
use lib::utils;
use std::path::PathBuf;

/// Returns a password from a record
/**
 * Blueprint
 *  1. `read_vault`, `vault.get_record`, bail on error
 *  2. Return the `password` field
 */
pub fn get_password(vault_path: &PathBuf, mut ctx: Context, record_id: &str) -> Result<(), Error> {
    tracepoint!();

    // (1)
    let vault = utils::read_vault(&vault_path, &mut ctx).unwrap();
    let record = vault.get_record(&record_id)?;

    // (2)
    println!("{}", record.password);

    Ok(())
}
