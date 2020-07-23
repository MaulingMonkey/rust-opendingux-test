# mipsel-gcw0-linux-uclibc.json

## Origin

```sh
rustc +nightly -Z unstable-options --print target-spec-json --target=mipsel-unknown-linux-gnu
```

## Modifications

```diff
   Not currently done:  This is "correct" but breaks the heck out of nix's libc imports (nix is a glutin dependency)
   TODO:  Fix nix, profit?
-  "env": "gnu",
+  "env": "uclibc",

   Apparently gcw0 uses hard floats, whereas the default mipsel-unknown-linux-musl target uses soft floats.
-  "features": "+mips32r2,+softfloat",
+  "features": "+mips32r2",

   Duh.
-  "is-builtin": true,
+  "is-builtin": false,

   We can hardcode the entire path and that's sufficient, no need to modify PATH
+  "linker": "/opt/gcw0-toolchain/usr/bin/mipsel-linux-gcc",

   Seems... more accurate, based on gcw0-toolchain layout?
-  "llvm-target": "mipsel-unknown-linux-gcc",
+  "llvm-target": "mipsel-gcw0-linux-uclibc",

   Seems harmless
-  "vendor": "unknown"
+  "vendor": "gcw0"
```



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

## .cargo/config

* https://doc.rust-lang.org/cargo/reference/config.html

## Xargo

* https://github.com/japaric/xargo

## Custom Targets

* https://rust-embedded.github.io/embedonomicon/custom-target.html
* https://doc.rust-lang.org/rustc/targets/custom.html

## RG350M

* CPU: Jz4770 https://en.wikipedia.org/wiki/Ingenic_Semiconductor

## Unrelated

* Target Runners: https://doc.rust-lang.org/cargo/reference/config.html#targettriplerunner
