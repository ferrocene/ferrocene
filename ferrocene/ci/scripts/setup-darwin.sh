#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
set -xeuo pipefail

# To build x86_64 artifacts, we run in Rosetta.
# Circle already has Rosetta for arm64 installed.
BREW_PATH="/opt/homebrew/bin/brew"
if [[ "$(uname -m)" = "x86_64" ]]; then
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    BREW_PATH="/usr/local/homebrew/bin/brew"
fi

# Unlike Windows/Linux executors, Macs do not come with awscli by default
# On Mac, XCode's LLVM cannot build for WASM.
${BREW_PATH} install awscli cmake ninja zstd llvm tidy-html5

# Needed for thumbv7em-none-eabihf & armv8r-none-eabihf cross-compilation
${BREW_PATH} install --cask gcc-arm-embedded

# There are a number of reports of very slow uploads in Mac VMs due to TSO, disable it.
# https://github.com/aws/aws-sdk/issues/469
# https://github.com/cypress-io/cypress/issues/28033#issuecomment-1879102700
# https://support.circleci.com/hc/en-us/articles/19334402064027-Troubleshooting-slow-uploads-to-S3-for-jobs-using-an-m1-macOS-resource-class
sudo sysctl net.inet.tcp.tso=0
