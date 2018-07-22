#!/bin/sh
RELEASE='v0.1.0'
FILENAME="./sigil-$RELEASE.tar.gz"
FOLDER="./sigil-$RELEASE"
BIN_DEST="/bin/sigil"

b=$(tput bold)
r=$(tput sgr0)
rr=$(tput setaf 1)
g=$(tput setaf 2)

echo "${b}${rr}THIS SCRIPT DOES NOT VERIFY THE FILE SIGNATURE$r"
echo "You may want to repeat this process manually and check that it wasn't tampered with"
sleep 5

echo ""
echo "${b}Downloading release $RELEASE from GitHub$r"
wget --quiet --show-progress "https://github.com/ALCC01/sigil/releases/download/$RELEASE/sigil-$RELEASE.tar.gz"

echo ""
echo "${b}Extracting release files$r"
tar xvf "$FILENAME"

echo ""
echo "${b}Copying bin files into PATH. This may require sudo privileges$r"
sudo mv "$FOLDER/sigil" "$BIN_DEST"
chmod +x "$BIN_DEST"

echo ""
echo "${b}Displaying changelog$r"
less "$FOLDER/CHANGELOG.md"

echo ""
echo "${b}${g}Done.$r Here's what you may still want to do:"
echo "$rr  - Export a SIGIL_VAULT path in your .bashrc$r"
echo "$rr  - Export a SIGIL_GPGKEY hint in your .bashrc$r"
echo "  - Generate a completion script in /etc/bash_completion.d using 'sigil completion'"
echo "  - 'sigil touch' your first vault!"
echo "${g}You can get more information and support at https://github.com/ALCC01/sigil$r"
echo ""

echo "${b}Cleaning up$r"
rm -r $FOLDER
rm $FILENAME