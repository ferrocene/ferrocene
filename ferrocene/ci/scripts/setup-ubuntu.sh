#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
set -xeo pipefail

# Ensure we never get asked/prompted, always take the new config
echo 'debconf debconf/frontend select Noninteractive' | sudo debconf-set-selections

sudo apt install -y \
    build-essential \
    ninja-build

if [[ ! -z "${INSTALL_LLVM}" ]]; then
    # LLVM 21 is not available in the default ubuntu 24.04 repositories, so we have to get them from http://apt.llvm.org/noble/
    wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key | sudo tee /etc/apt/trusted.gpg.d/apt.llvm.org.asc
    echo "deb http://apt.llvm.org/noble/ llvm-toolchain-noble-21 main" | sudo tee /etc/apt/sources.list.d/llvm-21.list
    sudo apt-get update
    sudo apt install -y \
        llvm-21-tools \
        llvm-21-dev \
        libpolly-21-dev
else
    echo 'Not installing LLVM, $INSTALL_LLVM is unset.'
fi

# Removing unused files to free some disk space (to avoid disk getting full)
if [[ "$GITHUB_ACTIONS" == "true" ]]; then
    df --human-readable
    sudo rm -rf /usr/local/.ghcup
    sudo rm -rf /opt/hostedtoolcache/CodeQL
    sudo rm -rf /usr/local/lib/android
    sudo rm -rf /opt/ghcup
    df --human-readable
fi
