#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
set -xeo pipefail

sudo apt install -y \
    git \
    build-essential \
    ninja-build \
    zlib1g-dev

if [[ ! -z "${INSTALL_LLVM}" ]]; then
    sudo apt install -y \
        llvm-17-tools \
        llvm-17-dev
else
    echo 'Not installing LLVM, $INSTALL_LLVM is unset.'
fi
