# mipsel-gcw0-linux-uclibc.json

## Origin

```sh
rustc +nightly -Z unstable-options --print target-spec-json --target=mipsel-unknown-linux-gnu
```

## Modifications

```diff
-  "env": "gnu",
+  "env": "uclibc",
```
The impact of this is a bit complicated.
On the one hand, the `nix` package is broken as heck on `target_env="uclibc"`.
`nix` imports a bunch of libc stuff which is just plain missing on uclibc, and thus fails to build.
(TODO: fix nix?)
On the other hand, `target_env="gnu"` is wrong and breaks `std::env::args()`!
Specifically, `init` [no longer](https://github.com/rust-lang/rust/blob/0820e54a8ad7795d7b555b37994f43cfe62356d4/src/libstd/sys/unix/args.rs#L102) calls `really_init`.
uclibc does call `.init_array.*` as [defined by rust's stdlib](https://github.com/rust-lang/rust/blob/0820e54a8ad7795d7b555b37994f43cfe62356d4/src/libstd/sys/unix/args.rs#L106-L126),
but the parameters are different or bogus!
Specifically:
* `argc` is either uninitialized or a pointer (large varying values per run)
* `argv` is either uninitialized or 0/nullptr (only ever observed to be 0 on my RG350M)
* `_envp`... heck if I know, I didn't check.



```diff
-  "features": "+mips32r2,+softfloat",
+  "features": "+mips32r2",
```
Apparently gcw0 uses hard floats, whereas the default mipsel-unknown-linux-musl target uses soft floats.

```diff
-  "is-builtin": true,
+  "is-builtin": false,
```
Duh.

```diff
+  "linker": "/opt/gcw0-toolchain/usr/bin/mipsel-linux-gcc",
```
gcw0-toolchain binaries assume it's been installed to `/opt/gcw0-toolchain`, so we might as well make the target assume the same.

```diff
-  "llvm-target": "mipsel-unknown-linux-gcc",
+  "llvm-target": "mipsel-gcw0-linux-uclibc",
```
Seems... more accurate, based on gcw0-toolchain layout?  Not entirely sure about the impact of changing "vendor" from "unknown' to "gcw0".

```diff
-  "vendor": "unknown"
+  "vendor": "gcw0"
```
Ditto


# Xargo.toml

Necessary to cause xargo to build std.

```toml
[target.mipsel-gcw0-linux-uclibc.dependencies.std]
features = []
```



# Reference

## Commands

```sh
rustc --target=mipsel-unknown-linux-gnu --print target-features
rustc +nightly -Z unstable-options --print target-spec-json --target=mipsel-unknown-linux-gnu
```

## Environment Variables

| Var                   | Value                         | Why |
| ----------------------| ------------------------------| ----|
| `RUST_TARGET_PATH`    | workspace                     | `xargo` can't find target spec json files otherwise (workspace related?)
| `RUSTUP_TOOLCHAIN`    | `nightly-2020-06-15`          | `xargo +nightly-2020-06-15` sadly doesn't work
| `PATH`                | `/opt/gcw0-toolchain/usr/bin` | Less necessary now that we have a target json spec
| `XARGO_RUST_SRC`      | `~/.rustup/toolchains/nightly-2020-06-15-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src` | Make a copy of that folder and modify it if you want to tweak `std`

## .cargo/config

* https://doc.rust-lang.org/cargo/reference/config.html

## Xargo

* https://github.com/japaric/xargo

## Custom Targets

* https://rust-embedded.github.io/embedonomicon/custom-target.html
* https://doc.rust-lang.org/rustc/targets/custom.html

# OpenDingux Toolchains

* http://www.gcw-zero.com/develop
* http://www.gcw-zero.com/updates
* https://boards.dingoonity.org/gcw-development/gcw-zero-toolchain-for-windows-(cygwin)-2013-10-04/

## RG350M

* CPU: Jz4770 https://en.wikipedia.org/wiki/Ingenic_Semiconductor

## Unrelated

* Target Runners: https://doc.rust-lang.org/cargo/reference/config.html#targettriplerunner

## OpenDinux OpenGL ES 2 Support

* http://www.gcw-zero.com/files/etnaviv-2013-10-04.html
* http://www.gcw-zero.com/updates
* https://github.com/laanwj/mesatest_gles
* https://github.com/laanwj/mesatest_gles/tree/master/Common
* https://github.com/laanwj/mesatest_gles/blob/master/Common/esUtil.c
* https://www.khronos.org/registry/EGL/sdk/docs/man/html/eglIntro.xhtml
* https://www.khronos.org/registry/EGL/specs/eglspec.1.4.pdf
* https://docs.rs/egl/0.2.7/egl/
