#!/bin/bash -eu
# The fuzz targets in fuzz/, built with the sanitizer ClusterFuzzLite asks for
cd "$SRC/pqc-binary-format"
cargo fuzz build -O
for target in from_bytes envelope; do
    cp "fuzz/target/x86_64-unknown-linux-gnu/release/$target" "$OUT/"
done
