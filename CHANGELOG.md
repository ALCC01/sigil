# Sigil
Only user facing changes should be included.

## Release series
* **v0.x**: Sigil Alpha

## v0.1.0
*Released on 2018/07/22*

* Adopt a GPG-encrypted TOML file as the vault format
    * By default its location is defined by the `SIGIL_VAULT` environment variable
    * By default it is encripted using the `SIGIL_GPGKEY` environment variable
    * Use `sigil touch` to create an empty vault
    * Use `sigil ls` to list all secrets in a vault
    * Use `sigil completion` to generate a completion script
* Support for password storing
    * Allow to annotate passwords with related usernames, emails and homepage URLs
    * Use `sigil password add` to add a password
    * Use `sigil password get` to retrieve a password
    * Use `sigil password rm` ro remove a password
* Support for OTP generator storing
    * Support TOTP and HOTP for token generation
    * Support SHA1, SHA256 and SHA512 as HMAC algorithms for token generation
    * Use `sigil otp add` to add a generator
    * Use `sigil otp token` to generate a token
    * Use `sigil otp rm` to remove a generator
    * Use `sigil otp import` to import generators from `otpauth://` URLs
