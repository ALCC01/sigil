use failure::{Error, Fail};
use lib::types::OtpRecord;
use lib::{error, otp, utils};
use std::path::PathBuf;

/// Adds an OTP record to the specified vault
/**
 * Blueprint
 *  1. Get the enc key from the user or the environment, otherwise bail
 *  2. Get the OTP record kind from the user (allow Hotp and Totp)
 *  3. Get the information necessary to construct a record from the user or from
 *     the args. Bail if mandatory info is not provided. Trim all strings.
 *      a) Hotp
 *          i) Secret: mandatory
 *          ii) Issuer
 *          iii) Algorithm: default to SHA1
 *          iv) Digits: default to 6
 *      b) Totp
 *          i) Secret: mandatory
 *          ii) Issuer
 *          iii) Algorithm: default to SHA1
 *          iv) Period: default to 30s
 *          v) Digits: default to 6
 *  4. Construct a `OtpRecord`
 *  5. Get a record ID from the user, bail if not provided
 *  6. `read_vault`, `vault::add_otp_record`, `write_vault`, bail on error
 */
// TODO Compact question boilerplate
pub fn add_record(vault: &PathBuf, key: Option<String>) -> Result<(), Error> {
    tracepoint!();
    let vault_path = vault.clone(); // `vault` will be shadowed later on

    // (1)
    // Acquire a GPGME context
    // TODO Can we handle this failure more nicely?
    let mut ctx = utils::create_context().unwrap();
    // A key can either be provided as an argument or an environment var
    let key = key.ok_or(error::NoKeyError())?;

    //(2)
    tracepoint!();
    let kind = question!("What kind of OTP should this record implement? (Hotp|Totp) ")?;
    let kind = kind.trim().to_ascii_lowercase();

    let record = match &kind[..] {
        // (2.a)
        "hotp" => {
            tracepoint!();

            // (2.a.i)
            let secret = question!("What is the secret? ")?;
            let secret = secret.trim().to_string();
            if secret.is_empty() {
                Err(error::MandatoryArgumentAbsentError().context("A secret must be provided"))?
            }

            // (2.a.ii)
            let issuer = question!("What is the issuer of this secret? [None] ")?;
            let issuer = issuer.trim();
            let issuer = if issuer.is_empty() {
                None
            } else {
                Some(issuer.to_owned())
            };

            // (2.a.iii)
            let algorithm = question!(
                "What algorithm should be used to generate secrets? ([SHA1]|SHA256|SHA512) "
            )?;
            let algorithm = algorithm.trim();
            let algorithm = if algorithm.is_empty() {
                "SHA1".to_owned()
            } else {
                algorithm.to_owned()
            };
            otp::string_to_algorithm(&algorithm)?;

            // (2.a.iv)
            let digits = question!("How many digits should a token be made of? [6] ")?;
            let digits = digits.trim();
            let digits: u32 = if digits.is_empty() {
                6
            } else {
                digits.parse()?
            };

            // (4)
            OtpRecord::Hotp {
                secret,
                issuer,
                algorithm,
                digits,
            }
        }
        // (2.b)
        "totp" => {
            tracepoint!();

            // (2.b.i)
            let secret = question!("What is the secret? ")?;
            let secret = secret.trim().to_string();
            if secret.is_empty() {
                Err(error::MandatoryArgumentAbsentError().context("A secret must be provided"))?
            }

            // (2.b.ii)
            let issuer = question!("What is the issuer of this secret? [None] ")?;
            let issuer = issuer.trim();
            let issuer = if issuer.is_empty() {
                None
            } else {
                Some(issuer.to_owned())
            };

            // (2.b.iii)
            let algorithm = question!(
                "What algorithm should be used to generate secrets? ([SHA1]|SHA256|SHA512) "
            )?;
            let algorithm = algorithm.trim();
            let algorithm = if algorithm.is_empty() {
                "SHA1".to_owned()
            } else {
                algorithm.to_owned()
            };
            otp::string_to_algorithm(&algorithm)?;

            // (2.b.iv)
            let period =
                question!("After how many seconds should a new token be generated? [30] ")?;
            let period = period.trim();
            let period: u64 = if period.is_empty() {
                30
            } else {
                period.parse()?
            };

            // (2.b.v)
            let digits = question!("How many digits should a token be made of? [6] ")?;
            let digits = digits.trim();
            let digits: u32 = if digits.is_empty() {
                6
            } else {
                digits.parse()?
            };

            // (4)
            OtpRecord::Totp {
                secret,
                issuer,
                algorithm,
                period,
                digits,
            }
        }
        _ => Err(error::OtpError::UnknownAlgorithm)?,
    };

    // (4)
    tracepoint!();
    let record_id = question!("What should this generator be called? ")?;
    let record_id = record_id.trim().to_owned();
    if record_id.is_empty() {
        Err(error::MandatoryArgumentAbsentError().context("A name must be provided"))?
    }

    // (5)
    // TODO These unwraps are due to the fact that the errors cannot be made
    // into failure::Error's. Find a workaround
    tracepoint!();
    let mut vault = utils::read_vault(&vault, &mut ctx).unwrap();
    vault.add_otp_record(record, record_id)?;
    utils::write_vault(&vault_path, &vault, &mut ctx, &key).unwrap();
    Ok(())
}
