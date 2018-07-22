#!/bin/bash
# Copyright (C) 2018 Alberto Coscia
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

TARGET_DIR="./target"
BUILD_DIR="./build"
BIN_NAME="sigil"
RELEASE_NAME="$BIN_NAME-$1"
RELEASE_DIR="$BUILD_DIR/$RELEASE_NAME"

INCLUDE=(
    "LICENSE"
    "CHANGELOG.md"
    "README.md"
)

init() {
    echo ""
    echo "Initializing..."
    rm -r "$BUILD_DIR"
    mkdir -p "$TARGET_DIR"
    mkdir -p "$RELEASE_DIR"
}

build() {
    echo "Building..."
    cargo +stable build --release
    strip --strip-debug "$TARGET_DIR/release/$BIN_NAME"
    cp -v "$TARGET_DIR/release/$BIN_NAME" "$RELEASE_DIR"
}

enrich() {
    echo ""
    echo "Copying additional files..."
    for src in "${INCLUDE[@]}"; do
        cp -v "./$src" "$RELEASE_DIR"
    done
}

sizes() {
    echo ""
    echo "Uncompressed build size:"
    ls -1sh "$RELEASE_DIR"
}

package() {
    echo ""
    echo "Packaging, compressing and signing..."
    cd $BUILD_DIR
    tar -czvf "$RELEASE_NAME.tar.gz" "$RELEASE_NAME/"
    gpg --sign --detach --armor "$RELEASE_NAME.tar.gz"
    gpg --verify "$RELEASE_NAME.tar.gz.asc" "$RELEASE_NAME.tar.gz"
    echo ""
    echo "Compressed build size:"
    ls -1sh "$RELEASE_NAME.tar.gz"
    cd ..
}

cleanup() {
    echo ""
    echo "Cleaning up"
    rm -r "$RELEASE_DIR"
}

init
build
enrich
sizes
package
cleanup