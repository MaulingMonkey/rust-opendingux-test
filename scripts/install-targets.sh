#!/usr/bin/env bash

echo "Checking toolchains..."
# nightly rust-src currently (7/22/2020) fails to untar due to Symlinks, so use nightly-2020-06-15 (~stable 1.45) instead
# rust-src is necessary for xargo to build core/std for our custom mipsel-gcw0-linux-uclibc target
# no targets are necessary since we bring our own
rustup toolchain list | grep nightly-2020-06-15 || rustup toolchain add nightly-2020-06-15 --profile minimal --component rust-src
#rustup toolchain list | grep stable  || rustup toolchain add stable
#rustup toolchain list | grep nightly || rustup toolchain add nightly --profile minimal --component rust-src

#echo ""
#echo "Checking stable targets..."
# stable mipsel-unknown-linux-gnu currently broken on 1.45 (missing symbols: `__stack_chk_fail`, `__stack_chk_guard`, `__snprintf_chk`)
#rustup target list --installed --toolchain=stable | grep mipsel-unknown-linux-gnu  || rustup target add mipsel-unknown-linux-gnu  --toolchain=stable
#rustup target list --installed --toolchain=stable  | grep mipsel-unknown-linux-musl || rustup target add mipsel-unknown-linux-musl --toolchain=stable

#echo ""
#echo "Checking nightly targets..."
#rustup target list --installed --toolchain=nightly | grep mipsel-unknown-linux-gnu  || rustup target add mipsel-unknown-linux-gnu  --toolchain=nightly
#rustup target list --installed --toolchain=nightly | grep mipsel-unknown-linux-musl || rustup target add mipsel-unknown-linux-musl --toolchain=nightly


echo ""
echo "Checking nightly-2020-06-15 components..."
rustup component list --installed --toolchain=nightly-2020-06-15 | grep rust-src || rustup component add rust-src --toolchain=nightly-2020-06-15
# no nightly-2020-06-15 targets:  we bring our own


echo ""
echo "Checking misc. tools..."
xargo --version 2>&1 | grep "xargo 0.3." || cargo install xargo --version "^0.3"
