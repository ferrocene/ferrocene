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
    wget https://apt.llvm.org/llvm.sh
    chmod +x llvm.sh
    sudo ./llvm.sh 20
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
