#!/bin/bash

# Rimuovi la cartella di rilascio Linux esistente
rm -rf "release/linux"

# Compila il progetto con cargo in modalità release per tutto il workspace
cargo build --release --workspace

# Crea la cartella release/linux se non esiste
if [ ! -d "release/linux" ]; then
    mkdir -p release/linux
fi

# Copia gli eseguibili nella directory release/linux
if [ -f "target/release/Group-35" ]; then
    cp target/release/Group-35 release/linux/Group-35
fi

if [ -f "target/release/main" ]; then
    cp target/release/main release/linux/main
fi

if [ -f "target/release/gui" ]; then
    cp target/release/gui release/linux/gui
fi

if [ -f "target/release/popup_gui" ]; then
    cp target/release/popup_gui release/linux/popup_gui
fi

if [ -f "target/release/uninstall" ]; then
    cp target/release/uninstall release/linux/uninstall
fi

echo "Build and copy process completed successfully for Linux."
