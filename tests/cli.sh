#!/usr/bin/env bats

setup() {
    TEMPDIR="$BATS_TEST_DIRNAME/tmp/test_$BATS_TEST_NUMBER"
    SIGIL=$BATS_TEST_DIRNAME/../target/debug/sigil
    mkdir -p $TEMPDIR
    export SIGIL_VAULT="$TEMPDIR/test.vault"
    export GPGKEY="Sigil CI"
}

teardown() {
    rm -r $TEMPDIR
}

move_input() {
    INPUT=$BATS_TEST_DIRNAME/inputs/$1
    cp $INPUT $SIGIL_VAULT.txt
    gpg --output "$SIGIL_VAULT" --yes --armor --recipient "$GPGKEY" --encrypt "$SIGIL_VAULT.txt"
    rm $SIGIL_VAULT.txt
}

compare_output() {
    OUTPUT=$BATS_TEST_DIRNAME/outputs/$1
    gpg --yes --no-comments --output "$SIGIL_VAULT".txt --decrypt "$SIGIL_VAULT"
    run diff -Bbw "$SIGIL_VAULT.txt" "$OUTPUT"
    echo $output
    [ "$status" -eq 0 ]
}

@test "run" {
    run SIGIL
    [ "$status" -eq 127 ]
}

@test "touch" {
    run $SIGIL touch
    echo $output
    [ "$status" -eq 0 ]
    compare_output "touch"
}

@test "password_add" {
    move_input "password_add"

    run $SIGIL password add Bob:service hunter2 -u bob --email bob@example.com --home https://service.tld
    echo $output
    [ "$status" -eq 0 ]
    
    compare_output "password_add"
}

@test "password_rm" {
    move_input "password_rm"

    run $SIGIL password rm Bob:service
    echo $output
    [ "$status" -eq 0 ]

    compare_output "password_rm"
}

@test "password_get" {
    move_input  "password_get"

    run $SIGIL password get Bob:service
    echo $output
    [ "$output" = "hunter2" ]
    [ "$status" -eq 0 ]
}

@test "password_generate" {
    run $SIGIL password generate 32
    echo $output
    [ "$status" -eq 0 ]
}

@test "otp_add_totp" {
    move_input "otp_add"

    run $SIGIL otp add --totp Bob:service GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ --issuer service
    echo $output
    [ "$status" -eq 0 ]
    
    compare_output "otp_add_totp"
}

@test "otp_add_hotp" {
    move_input "otp_add"

    run $SIGIL otp add --hotp Bob:service GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ --issuer service
    echo $output
    [ "$status" -eq 0 ]
    
    compare_output "otp_add_hotp"
}

@test "otp_import" {
    move_input "otp_add"

    run $SIGIL otp import "otpauth://totp/Bob:service?issuer=service&secret=GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ"
    #echo $output
    [ "$status" -eq 0 ]
    
    compare_output "otp_add_totp"
}

@test "otp_rm" {
    move_input "otp_rm"

    run $SIGIL otp rm Bob:service
    echo $output
    [ "$status" -eq 0 ]
    
    compare_output "otp_rm"
}

@test "otp_token_totp" {
    move_input "otp_token_totp"

    EXPECTED=$(oathtool --totp --base32 GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ)
    run $SIGIL otp token Bob:service
    echo $output
    echo $EXPECTED
    [ "$status" -eq 0 ]
    [ "$output" == $EXPECTED ]
}

@test "otp_token_hotp" {
    move_input "otp_token_hotp"

    EXPECTED=$(oathtool --hotp --base32 GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ --counter 51064264)
    run $SIGIL otp token Bob:service 51064264
    echo $output
    echo $EXPECTED
    [ "$status" -eq 0 ]
    [ "$output" == $EXPECTED ]
}