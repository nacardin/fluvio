#!/bin/bash
# set up sccache
set -e
MATRIX_OS=${1}
echo "installing zig matrix.os=$MATRIX_OS"

if [[ "$MATRIX_OS" == "ubuntu-latest" ]]; then
    echo "installing zig on ubuntu"
    tmp_dir=$(mktemp -d)
    curl -S https://ziglang.org/download/0.9.1/zig-linux-x86_64-0.9.1.tar.xz | tar -xJ -C $tmp_dir
    sudo install $tmp_dir/zig-linux-x86_64-0.9.1/zig /usr/local/bin
    sudo ${0%/*}/llvm.sh 13 && \
    echo "FLUVIO_BUILD_LLD=lld-13" | tee -a $GITHUB_ENV
fi

if [[ "$MATRIX_OS" == "macos-11" ]]; then
    echo "installing zig on mac"
 #   brew update
    brew install zig && \
    echo "FLUVIO_BUILD_LLD=/opt/homebrew/opt/llvm@13/bin/lld" | tee -a $GITHUB_ENV
fi


          