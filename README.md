# Command used to generate the bindings:

bindgen ../ksw2/ksw2.h  --allowlist-function "ksw_.*" > lib.rs

