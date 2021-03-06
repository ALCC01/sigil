// Copyright (C) 2018 Alberto Coscia
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base32;
use lib::types::HmacAlgorithm;
use ring::{digest, hmac};
use std::mem::transmute;
use std::time::{SystemTime, UNIX_EPOCH};

pub type Algorithm = &'static digest::Algorithm;

/// Computes an N-digits OTP using the TOTP algorithm as laid out in
/// [IETF RFC 6238](https://tools.ietf.org/html/rfc6238).
///
/// Returns the remaining token validity time as u64.
/**
 * Blueprint
 *  0. Let `T0` be a Unix timestamp and `TI` a period, both expressed in the same
 *     unit of measurement. Let `K` be a base32-encoded secret. Let `N` be the
 *     token length. We future-proof this by always using 64-bit values for time
 *     so that we won't have any trouble when 2038 comes around (RFC 6238 $4.2)
 *  1. C := floor((now - T0)/TI) as u64
 *  2. Return HOTP(K, C)
 */
pub fn totp(T0: u64, TI: u64, K: &str, N: u32, algorithm: &HmacAlgorithm) -> (u32, u64) {
    tracepoint!();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    totp_with_now(T0, TI, K, N, now, &algorithm)
}

/// Computes an N-digits OTP using the TOTP algorithm as laid out in
/// [IETF RFC 6238](https://tools.ietf.org/html/rfc6238).
/// Helper method that allows testing
///
/// Returns the remaining token validity time as u64.
pub fn totp_with_now(
    T0: u64,
    TI: u64,
    K: &str,
    N: u32,
    now: u64,
    algorithm: &HmacAlgorithm,
) -> (u32, u64) {
    tracepoint!();
    // (1)
    // Cast to f64 so that we can have the precision necessary to use floor as
    // specified in RFC 6238 §4.2
    let C = ((now as f64 - T0 as f64) / TI as f64).floor() as u64;

    // (2)
    tracepoint!();
    (hotp(K, C, N, &algorithm), TI - ((now - T0) % TI))
}

/// Computes an N-digits OTP using the HOTP algorithm as laid out in
/// [IETF RFC 4226](https://tools.ietf.org/html/rfc4226.html).
///
/// Note that RFC 4226 only allows for SHA-1 to be used, but SHA-2 is allowed by
/// RFC 6238
/**
 * Blueprint
 *  0. Let `K` be a base32-encoded secret. Let `C` be a counter. Let `N` be the
 *     token length.
 *  1. Decode `K` to a Vec<u8>, bail if it is not valid base32
 *  2. H := HMAC(K, C) using `algorithm`
 *  3. O := least 4 significant bits of H
 *  4. Take 4 bytes from `H` starting at the O'th most significant byte, discard
 *     the MSB bit (may be interpreted as the sign bit in a signed integer) and
 *     store the rest as u32
 *  5. Return only `N` digits
 * */
pub fn hotp(K: &str, C: u64, N: u32, algorithm: &HmacAlgorithm) -> u32 {
    tracepoint!();
    // (1)
    // Failing here is not recoverable -- the user did something wrong
    let K = base32::decode(base32::Alphabet::RFC4648 { padding: false }, K).unwrap();

    // (2)
    tracepoint!();
    let K = hmac::SigningKey::new(algorithm.to_algorithm(), K.as_ref());
    // TODO Use int_to_from_bytes (see Rust PR #51835) when stabilized
    // in Rust 1.29. When using it, .swap_bytes() because of endianess
    debug!("Counter ({}) is {:?}", C, u64_into_bytes(C));
    let H = hmac::sign(&K, &u64_into_bytes(C));
    debug!(
        "Signed digest is {}",
        H.as_ref()
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<String>()
    );
    // &[u8] -> Vec<u8>
    let mut vec: Vec<u8> = Vec::new();
    vec.extend_from_slice(H.as_ref());
    let H = vec;

    // (3)
    tracepoint!();
    let O = *H.last().unwrap() as usize & 0xf;
    debug!("Offset is {}", O);

    // (4)
    // 0x7f masks the MSB bit
    tracepoint!();
    let decimal = (u32::from(H[O]) & 0x7f) << 24
        | u32::from(H[O + 1]) << 16
        | u32::from(H[O + 2]) << 8
        | u32::from(H[O + 3]);
    debug!("Decimal for {} is {}", C, decimal);

    // (5)
    tracepoint!();
    decimal % 10u32.pow(N)
}

#[cfg(test)]
mod tests {
    use lib::otp;
    use lib::types::HmacAlgorithm;

    // Test values provided in RFC 4226
    // Base32 for "12345678901234567890";
    const RFC_HOTP_SECRET: &str = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
    const RFC_HOTP_VALUES: &[u32; 10] = &[
        755224, 287082, 359152, 969429, 338314, 254676, 287922, 162583, 399871, 520489,
    ];

    #[test]
    fn hotp_rfc_values() {
        for value in 0..RFC_HOTP_VALUES.len() {
            assert_eq!(
                otp::hotp(&RFC_HOTP_SECRET, value as u64, 6, &HmacAlgorithm::SHA1),
                RFC_HOTP_VALUES[value as usize]
            );
        }
    }

    // Test values provided in RFC 6238
    const RFC_TOTP_TIMES: &[u64; 6] = &[
        59,
        1111111109,
        1111111111,
        1234567890,
        2000000000,
        20000000000,
    ];

    // Base32 for "12345678901234567890"
    const RFC_TOTP_SECRET_SHA1: &str = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
    const RFC_TOTP_VALUES_SHA1: &[u32; 6] =
        &[94287082, 07081804, 14050471, 89005924, 69279037, 65353130];

    #[test]
    fn totp_rfc_values_sha1() {
        for value in 0..RFC_TOTP_TIMES.len() {
            let (v, _) = otp::totp_with_now(
                0,
                30,
                &RFC_TOTP_SECRET_SHA1,
                8,
                RFC_TOTP_TIMES[value],
                &HmacAlgorithm::SHA1,
            );
            assert_eq!(v, RFC_TOTP_VALUES_SHA1[value as usize]);
        }
    }

    // See RFC errata https://www.rfc-editor.org/errata/eid2866
    // Base32 for "12345678901234567890123456789012"
    const RFC_TOTP_SECRET_SHA256: &str = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA";
    const RFC_TOTP_VALUES_SHA256: &[u32; 6] =
        &[46119246, 68084774, 67062674, 91819424, 90698825, 77737706];

    #[test]
    fn totp_rfc_values_sha256() {
        for value in 0..RFC_TOTP_TIMES.len() {
            let (v, _) = otp::totp_with_now(
                0,
                30,
                &RFC_TOTP_SECRET_SHA256,
                8,
                RFC_TOTP_TIMES[value],
                &HmacAlgorithm::SHA256,
            );
            assert_eq!(v, RFC_TOTP_VALUES_SHA256[value as usize]);
        }
    }

    // See RFC errata https://www.rfc-editor.org/errata/eid2866
    // Base32 for "1234567890123456789012345678901234567890123456789012345678901234"
    const RFC_TOTP_SECRET_SHA512: &str = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNA";
    const RFC_TOTP_VALUES_SHA512: &[u32; 6] =
        &[90693936, 25091201, 99943326, 93441116, 38618901, 47863826];

    #[test]
    fn totp_rfc_values_sha512() {
        for value in 0..RFC_TOTP_TIMES.len() {
            let (v, _) = otp::totp_with_now(
                0,
                30,
                &RFC_TOTP_SECRET_SHA512,
                8,
                RFC_TOTP_TIMES[value],
                &HmacAlgorithm::SHA512,
            );
            assert_eq!(v, RFC_TOTP_VALUES_SHA512[value as usize]);
        }
    }
}

/// Converts u64 to a u8 array
/// Fallback method waiting for stabilization of int_to_from_bytes
/// (see Rust PR #51835) in Rust 1.29
fn u64_into_bytes(x: u64) -> [u8; 8] {
    unsafe { transmute(x.to_be()) }
}
