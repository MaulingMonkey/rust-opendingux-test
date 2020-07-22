echo "Checking toolchains..."
rustup toolchain list | grep stable  || rustup toolchain add stable
rustup toolchain list | grep nightly || rustup toolchain add nightly

echo ""
echo "Checking stable targets..."
# stable mipsel-unknown-linux-gnu currently broken on 1.45 (missing symbols: `__stack_chk_fail`, `__stack_chk_guard`, `__snprintf_chk`)
#rustup target list --installed --toolchain=stable | grep mipsel-unknown-linux-gnu  || rustup target add mipsel-unknown-linux-gnu  --toolchain=stable
rustup target list --installed --toolchain=stable  | grep mipsel-unknown-linux-musl || rustup target add mipsel-unknown-linux-musl --toolchain=stable

echo ""
echo "Checking nightly targets..."
rustup target list --installed --toolchain=nightly | grep mipsel-unknown-linux-gnu  || rustup target add mipsel-unknown-linux-gnu  --toolchain=nightly
rustup target list --installed --toolchain=nightly | grep mipsel-unknown-linux-musl || rustup target add mipsel-unknown-linux-musl --toolchain=nightly
