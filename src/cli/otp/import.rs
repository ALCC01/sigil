use failure::Error;
use lib::types::OtpRecord;
use lib::{error, otp, utils};
use std::collections::HashMap;
use std::path::PathBuf;
use url::Url;

/// Imports a record from a
/// [otpauth:// URL](https://github.com/google/google-authenticator/wiki/Key-Uri-Format)
pub fn import_url(vault: &PathBuf, key: Option<String>, url: &str) -> Result<(), Error> {
    tracepoint!();
    let vault_path = vault.clone(); // `vault` will be shadowed later on

    // (1)
    // Acquire a GPGME context
    // TODO Can we handle this failure more nicely?
    let mut ctx = utils::create_context().unwrap();
    let key = key.ok_or(error::NoKeyError())?;

    let url = Url::parse(&url)?;

    ensure!(
        url.scheme().eq_ignore_ascii_case("otpauth"),
        "Invalid URL scheme"
    );
    let segments = url
        .path_segments()
        .map(|c| c.collect::<Vec<&str>>())
        .unwrap();
    ensure!(segments.len() == 1, "Expected 1 path segment");

    let kind = url.host_str().unwrap();
    let record_id = segments[0].to_string();

    let mut query: HashMap<String, String> = HashMap::new();
    let pairs = url.query_pairs();

    for pair in pairs {
        query.insert(pair.0.to_string(), pair.1.to_string());
    }

    let secret = (&query["secret"]).to_string();
    let issuer = query.get("issuer").map(|n| n.to_string());

    let algorithm = query
        .get("algorithm")
        .map(|n| n.to_string().to_ascii_uppercase())
        .unwrap_or_else(|| "SHA1".to_string());
    otp::string_to_algorithm(&algorithm)?;

    let digits: u32 = query
        .get("digits")
        .unwrap_or(&"6".to_string())
        .trim()
        .parse()?;

    let period: u64 = query
        .get("period")
        .unwrap_or(&"30".to_string())
        .trim()
        .parse()?;

    let record = match &kind.to_ascii_lowercase()[..] {
        "totp" => OtpRecord::Totp {
            secret,
            issuer,
            algorithm,
            digits,
            period,
        },
        "hotp" => OtpRecord::Hotp {
            secret,
            issuer,
            algorithm,
            digits,
        },
        _ => unimplemented!(),
    };

    // (5)
    // TODO These unwraps are due to the fact that the errors cannot be made
    // into failure::Error's. Find a workaround
    tracepoint!();
    let mut vault = utils::read_vault(&vault, &mut ctx).unwrap();
    vault.add_otp_record(record, record_id)?;
    utils::write_vault(&vault_path, &vault, &mut ctx, &key).unwrap();

    Ok(())
}
