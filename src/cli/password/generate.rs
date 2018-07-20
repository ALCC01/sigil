// Copyright (C) 2018 Alberto Coscia
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use failure::Error;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Generates a random password
pub fn generate_password(chars: usize) -> Result<(), Error> {
    let mut random = thread_rng();
    let pw: String = random.sample_iter(&Alphanumeric).take(chars).collect();
    println!("{}", pw);

    Ok(())
}
