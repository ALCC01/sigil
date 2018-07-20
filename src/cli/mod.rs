// Copyright (C) 2018 Alberto Coscia
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// A macro to ask, validate and possibly re-ask the user a question
macro_rules! question {
    ($validator:expr, $($arg:tt)*) => {
        {
            use std::io;
            use std::io::Write;
            // Until we get an error we can't recover OR the user gives us an
            // answer we can accept
            loop {
                // Answer will be written to this string
                let mut temp_buf = String::new();
                // Print the question
                print!($($arg)*);
                // Read a line
                let res = io::stdout().flush().and(io::stdin().read_line(&mut temp_buf).and(Ok(temp_buf)));
                // If we're successfull
                if res.is_ok() {
                    let res : String = res.unwrap();
                    // Run the validator
                    let validated : Result<_, _> = $validator(res.trim().to_string());
                    if validated.is_ok() {
                        break validated;
                    } else {
                        // Start over
                        println!("{}", validated.err().unwrap());
                    }
                } else {
                    // Can't recover from this
                    break Err(format_err!("IO error"));
                }
            }
        }
    };
}

pub mod args;
pub mod list;
pub mod otp;
pub mod password;
pub mod touch;
