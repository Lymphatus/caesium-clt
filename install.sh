#!/bin/sh

#libcaesium
git clone https://github.com/Lymphatus/libcaesium.git
cd libcaesium || exit
cargo build --release
