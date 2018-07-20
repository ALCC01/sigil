// Copyright (C) 2018 Alberto Coscia
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug, Fail)]
pub enum VaultError {
    #[fail(display = "Record should be updated, not added")]
    ShouldUpdate,
    #[fail(display = "Failed to find a matching record")]
    UnknownRecord,
}

#[derive(Debug, Fail)]
pub enum OtpError {
    #[fail(display = "No counter was provided")]
    NoCounterProvided,
    #[fail(display = "Unknown HMAC algorithm")]
    UnknownHmacAlgorithm,
}
