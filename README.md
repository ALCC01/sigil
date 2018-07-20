# Sigil [![Build Status][travis-badge]][travis]
*A password manager for the [sneakernet][sneakernet-ud]*.

Sigil is a secure, [PGP][pgp]-backed password manager for your command line. It 
allows you to store your secrets inside an encrypted vault that can only be opened
using your PGP key. You can also use it to store OTP generators and retrieve
tokens whenever you like.

It is as resilient as text files: a Sigil vault is a plaintext [TOML][toml] file
that can be decrypted using any PGP implementation of you choice. This means
your secrets will always remain at your disposal and under your control, even
when your fancy, cloud-based password manager [shuts down][mitro] or you can't
[access][border-seizure]/[trust][chinese-border] your phone. In fact, even if
Sigil were to go away for whatever reason, your passwords would still be a mere
`gpg --decrypt` away.

You could throw your vault in a git repository or FTP server (or even take your
chances with [Dropbox][dropbox-rice] or Google Drive) and have your secrets
sync throughout your devices, all with ease of mind that they are safe. As a
rule of thumb, wherever a file will go, so will your vault.

- [Sigil [![Build Status][travis-badge]][travis]](#sigil-build-statustravis-badgetravis)
    - [Features](#features)
    - [Why's](#whys)
        - [Why PGP?](#why-pgp)
        - [Why not any other cloud-based, hassle-free password manager?](#why-not-any-other-cloud-based-hassle-free-password-manager)
        - [Why not [`pass`][pass]?](#why-not-passpass)
        - [Why OTPs on a PC?](#why-otps-on-a-pc)
    - [Getting started](#getting-started)
        - [Creating a vault](#creating-a-vault)
        - [Adding a password](#adding-a-password)
        - [Retrieving a password](#retrieving-a-password)
        - [Removing a password](#removing-a-password)
        - [Generating a password](#generating-a-password)
        - [Adding an OTP generator](#adding-an-otp-generator)
        - [Retrieving an OTP token](#retrieving-an-otp-token)
        - [Importing an OTP token from `otpauth://` URLs](#importing-an-otp-token-from-otpauth-urls)
        - [Removing an OTP generator](#removing-an-otp-generator)

## Features
* **Secure**: as much as OpenPGP and your system
* **Lasting**: if text files are not going away, neither are your secrets
* **Portable**: syncs through the [sneakernet][sneakernet-ud]
* **Auditable**: as open source software, you're free to inspect, audit and
build Sigil on your own

## Why's
### Why PGP?
Because it has been around for a (long) while, it is battle-tested and is
available on many platforms. Plus, its implementation is a mission-critical 
piece of software that can be rely on the experience of a [community of
experts][xkcd-crypto].

It also has the advantage of being quite widespread and so it would be safe
to assume that a large chunk of the target audience already has and is confident
in using a PGP key.

### Why not any other cloud-based, hassle-free password manager?
Because they can easily be made unavailable by a malicious actor or just reveal 
to be less lasting than you thought and shut down, leaving you with a bunch of
data in a proprietary format. Or maybe because in some jurisdictions they may be 
compelled to reveal metadata relating to your usage -- or worse, your secrets! --
or you are not confident that such a leak may just be caused by a bona fide 
implementation error.

### Why not [`pass`][pass]?
There really isn't an answer here. `pass` is just as fine, but you may find it
troublesome handling, moving and syncing its directory-based structure. In the
end, Sigil follows the same philosophy of security and composeability. Tab
completion for password names is something that is not feasible using Sigil,
though.

### Why OTPs on a PC?
The principle behind two factor authentication is combining something you *know*
(a password, even though we're cheating here) and something you *own* (such as a
phone). The thing here is that you own you PC just as much as your phone, it's 
even arguable that you may have (and be able to maintain) *more* control over
your computer than over an easily stolen/reset/bricked/
[unlocked][apple-bernardino] phone. 

Furthermore, the chances of a successfull remote attack and exfiltration of a 
PGP-encrypted file -- plus your private, somehow decripted key -- against your 
computer should be pretty much the same as those of such an attack against your
phone, so the issue really boils down to which device you feel more confident
in *physically* protecting. There's no actual reason to straight out prefer
your phone over your PC.

If you want to be extra sure, you may also combine your vault with an external,
FIDO2-like authenticator holding your PGP key.

## Getting started
* Sigil is currently developed using Rust 1.28, you can use [rustup.rs][rustup]
to install it alongside with Cargo
* `cargo install --git https://github.com/ALCC01/sigil`
* Make sure your `PATH` contains `$HOME/.cargo/bin`
* Sigil also relies on GPGME as provided by `libgpgme11-dev`, which is available
on many Linux distros
* In your `.bashrc` file (or its equivalent for your shell of choice)
    * Add `export SIGIL_VAULT="$HOME/.sigil.vault"` or whatever path you want
your vault to be written to
    * Add `export SIGIL_GPGKEY="me@example.com"` or pretty much that could be
used as a `--recipient` with `gpg --encrypt`
    * Then `source` it again (or restart your shell)
* The following instructions will assume that you export these environment 
variables, otherwise you'll neet to use the `--vault` and `--key` arguments
* You may also want to **avoid your shell saving your password in its history** 
when you provide it as a command line argument (using `bash` this is possible
adding `HISTIGNORE="$HISTIGNORE:sigil *"` to your `.bashrc` file). If feasible,
you may want to avoid using the CLI arguments altogether and rely on the 
interactive mode.

### Creating a vault
You can create your vault using `sigil touch`. And you're done.

### Adding a password
You can store a password in your vault using the `sigil password add` command,
either providing the relevant arguments (use the `--help` option for info) or
providing none and following the interactive setup.

### Retrieving a password
Retrieving your password is just as easy as using the `sigil password get <name>`
command. Don't remember the name you assigned to the password? `sigil ls`.

### Removing a password
`sigil password rm <name>` and then it's gone.

### Generating a password
Sigil provides the `sigil password generate <chars>` utility command to generate
random passwords of `chars` length.

### Adding an OTP generator
You can store a password in your vault using the `sigil otp add` command,
either providing the relevant arguments (use the `--help` option for info) or
providing none and following the interactive setup.

### Retrieving an OTP token
You can generate an OTP token using `sigil otp token <name> <counter>`. 
`<counter>` is only needed for HOTP generators. Don't remember the name you
assigned to the generator? `sigil ls`.

### Importing an OTP token from `otpauth://` URLs
Many services will issue you with a `otpauth://` URL (or its QR representation).
You can import such an URL using `sigil otp import <url>`.

### Removing an OTP generator
You can banish it out of existence using `sigil otp remove <name>`.

[pgp]: https://tools.ietf.org/html/rfc4880
[travis]: https://travis-ci.com/ALCC01/sigil
[travis-badge]: https://travis-ci.com/ALCC01/sigil.svg?token=VQqRdWniwWscxaAK7t7z&branch=master
[sneakernet-ud]: https://www.urbandictionary.com/define.php?term=Sneakernet
[toml]: https://github.com/toml-lang/toml
[mitro]: https://venturebeat.com/2015/07/11/twitter-will-shut-down-password-manager-mitro-on-aug-31-after-buying-it-last-year/
[border-seizure]: https://www.eff.org/press/releases/eff-aclu-media-conference-call-today-announce-lawsuit-over-warrantless-phone-and
[chinese-border]: https://www.reddit.com/r/security/comments/8ofiiw/chinese_border_police_installed_software_on_my/
[dropbox-rice]: https://techcrunch.com/2014/04/11/dropbox-promises-adding-condoleezza-rice-to-its-board-wont-change-its-privacy-views/
[xkcd-crypto]: https://xkcd.com/153/
[pass]: https://www.passwordstore.org/
[rustup]: https://rustup.rs/
[histcontrol]: https://stackoverflow.com/questions/6475524/how-to-prevent-commands-to-show-up-in-bash-history
[apple-bernardino]: https://www.bloomberg.com/news/articles/2016-02-17/apple-has-the-way-but-not-the-will-to-help-fbi-hack-your-phone
