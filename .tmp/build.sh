#!/bin/bash
set -e

NAME=void

export LD_LIBRARY_PATH+=:$(rustc --print=sysroot)/lib
export RUSTFLAGS="-C prefer-dynamic"

cargo build --release \
&& upx --best --lzma ./target/release/${NAME} \
&& echo "Compressed build complete." \
&& ls -lh --color=auto ./target/release/${NAME}
