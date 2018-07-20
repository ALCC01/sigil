// Copyright (C) 2018 Alberto Coscia
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod add;
mod generate;
mod get;
mod remove;

pub use self::add::add_record;
pub use self::add::add_record_interactive;
pub use self::generate::generate_password;
pub use self::get::get_password;
pub use self::remove::remove_record;
