#!/usr/bin/env sh
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -ex

ARCH=$1
TARGET=$ARCH-linux-musl
OUTPUT=/usr/local
MUSL_VER=1.2.5
DL_CMD="curl -C - -L -o"

git clone https://github.com/richfelker/musl-cross-make
cd musl-cross-make
git checkout fd6be58297ee21fcba89216ccd0d4aca1e3f1c5c # v0.9.11

# we need to build the code as relocatable / position independent (-fPIC) or else we get
# linker errors with the x86_64 MUSL target. the `-g1` flag lets backtraces cross into C libraries
export CFLAGS="-fPIC -g1 $CFLAGS"

make -j$(nproc) TARGET=$TARGET MUSL_VER=$MUSL_VER DL_CMD="$DL_CMD"
make install TARGET=$TARGET MUSL_VER=$MUSL_VER DL_CMD="$DL_CMD" OUTPUT=$OUTPUT

cd -
