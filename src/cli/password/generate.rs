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
