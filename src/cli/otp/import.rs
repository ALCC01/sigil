use failure::Error;
use gpgme::Context;
use lib::types::{HmacAlgorithm, OtpRecord};
use lib::utils;
use std::collections::HashMap;
use std::path::PathBuf;
use url::Url;

/// Imports a record from a
/// [otpauth:// URL](https://github.com/google/google-authenticator/wiki/Key-Uri-Format)
/**
 * Blueprint
 *  1. Parse the URL according to spec, bail on error
 *  2. Construct a `OtpRecord`
 *  3. `read_vault`, `vault::add_otp_record`, `write_vault`, bail on error
 */
pub fn import_url(
    vault_path: &PathBuf,
    key: &str,
    mut ctx: Context,
    url: &str,
) -> Result<(), Error> {
    tracepoint!();

    // (1)
    let url = Url::parse(&url)?;
    // Can't work on other schemes
    ensure!(
        url.scheme().eq_ignore_ascii_case("otpauth"),
        "Invalid URL scheme"
    );

    // TOTP/HOTP is the host
    let kind = url.host_str().unwrap();
    // The first and only segment is this record's label
    let segments = url
        .path_segments()
        .map(|c| c.collect::<Vec<&str>>())
        .unwrap();
    ensure!(segments.len() == 1, "Expected 1 path segment");
    let record_id = segments[0].to_string();

    // Iterator -> HashMap
    let mut query: HashMap<String, String> = HashMap::new();
    let pairs = url.query_pairs();
    for pair in pairs {
        query.insert(pair.0.to_string(), pair.1.to_string());
    }

    // Parse query string
    let secret = (&query["secret"]).to_string();
    let issuer = query.get("issuer").map(|n| n.to_string());

    let algorithm: HmacAlgorithm = query
        .get("algorithm")
        .map(|n| n.to_string().to_ascii_uppercase())
        .unwrap_or_else(|| "SHA1".to_string())
        .parse()?;

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

    // (2)
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

    // (3)
    // TODO These unwraps are due to the fact that the errors cannot be made
    // into failure::Error's. Find a workaround
    let mut vault = utils::read_vault(&vault_path, &mut ctx).unwrap();
    vault.add_otp_record(record, record_id)?;
    utils::write_vault(&vault_path, &vault, &mut ctx, &key).unwrap();

    Ok(())
}
