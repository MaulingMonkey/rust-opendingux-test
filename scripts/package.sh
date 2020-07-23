#!/usr/bin/env bash

pushd $(dirname $0) >/dev/null
cd ..

cp target/nightly-uclibc/mipsel-gcw0-linux-uclibc/debug/rust-opendingux-test    package/rust-opendingux-test.debug
cp target/nightly-uclibc/mipsel-gcw0-linux-uclibc/release/rust-opendingux-test  package/rust-opendingux-test.release
mksquashfs package target/package.opk -comp gzip -noappend

popd >/dev/null
