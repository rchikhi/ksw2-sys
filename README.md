# ksw2-sys

Rust bindings for the fast DNA alignment library ksw2: https://github.com/lh3/ksw2

See https://github.com/rchikhi/rust-alignbench for an example usage

# Command used to generate the bindings:

`cd src && bindgen ../ksw2/ksw2.h  --allowlist-function "ksw_.*" > lib.rs`

