#!/usr/bin/env bash

pushd $(dirname $0) >/dev/null
cd ..

scp target/package.opk root@10.1.1.2:/media/sdcard/apps/rust_opendingux_test.opk

popd >/dev/null
