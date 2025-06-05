#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
set -xeuo pipefail

# Unlike Windows/Linux executors, Macs do not come with awscli by default
# On Mac, XCode's LLVM cannot build for WASM.
brew install --quiet awscli cmake ninja zstd llvm tidy-html5

# Install Azure tools
brew install --quiet azure-cli azcopy

# Needed for thumbv7em-none-eabihf & armv8r-none-eabihf cross-compilation
# This one is squelched otherwise it emits a warning annotation on GHA
# Using `... 2> >(sed -e "s/Warning://g")` was attempted but it did not resolve it.
if [[ "$GITHUB_ACTIONS" == "true" ]]; then
    stopMarker="endtoken-brew-install-gcc-$(uuidgen)"
    echo "::stop-commands::{$stopMarker}"
fi
brew install --quiet gcc-arm-embedded  &> /dev/null
if [[ "$GITHUB_ACTIONS" == "true" ]]; then
    echo "::{$stopMarker}::"
fi

# There are a number of reports of very slow uploads in Mac VMs due to TSO, disable it.
# https://github.com/aws/aws-sdk/issues/469
# https://github.com/cypress-io/cypress/issues/28033#issuecomment-1879102700
# https://support.circleci.com/hc/en-us/articles/19334402064027-Troubleshooting-slow-uploads-to-S3-for-jobs-using-an-m1-macOS-resource-class
sudo sysctl net.inet.tcp.tso=0

# Free up some disk space
# Mac runners have a paltry 14gb of storage, see: https://github.com/actions/runner-images/issues/2840
if [[ "$GITHUB_ACTIONS" == "true" ]]; then
    echo "Freeing up disk space on the Github Actions runner, current space:"
    df -h
    # Remove random cruft
    sudo rm -rf \
        /opt/google/chrome \
        /opt/microsoft/msedge \
        /opt/microsoft/powershell \
        /opt/pipx \
        /opt/ghc \
        /usr/lib/mono \
        /usr/local/julia* \
        /usr/local/lib/android \
        /usr/local/lib/node_modules \
        /usr/local/share/chromium \
        /usr/local/share/powershell \
        /usr/share/dotnet \
        /usr/share/swift \
        ~/.dotnet \
        /Library/Android \
        /Library/Developer/CoreSimulator
    # Remove old xcodes, we only need 1.
    find /Applications/ -name "Xcode*" | sort -r | tail --lines=+2 | xargs rm -rf
    echo "Post-free disk space:"
    df -h
else
    echo "Not freeing up space, this is not a Github Actions runner."
fi
