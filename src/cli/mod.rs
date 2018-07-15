/// A macro for asking a question to the user
macro_rules! question {
    ($($arg:tt)*) => {
        {
            use std::io;
            use std::io::Write;
            let mut temp_buf = String::new();
            print!($($arg)*);
            io::stdout().flush().and(io::stdin().read_line(&mut temp_buf).and(Ok(temp_buf)))
        }
    };
}

pub mod args;
pub mod list;
pub mod otp;
pub mod password;
pub mod touch;
