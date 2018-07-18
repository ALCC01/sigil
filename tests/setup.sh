#!/usr/bin/env sh

# This will most probably do things to your environment that you may regret later
if [ $CI ]; then
    gpg --batch --gen-key ./tests/keygen
    export GPGKEY="Sigil CI"
fi