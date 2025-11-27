#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
set -xeuo pipefail

if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
    echo "::group::Install dependencies (macOS)"
fi

# Unlike Windows/Linux executors, Macs do not come with awscli by default
# On Mac, XCode's LLVM cannot build for WASM.
brew install --quiet awscli cmake ninja zstd llvm tidy-html5

# Install Azure tools
brew install --quiet azure-cli azcopy

# Needed for thumbv7em-none-eabihf & armv8r-none-eabihf cross-compilation
# brew install --cask  gcc-arm-embedded
mkdir -p /tmp/ferrocene
curl -Lo /tmp/ferrocene/arm-gnu-toolchain-14.3.rel1-darwin-arm64-arm-none-eabi.pkg https://developer.arm.com/-/media/Files/downloads/gnu/14.3.rel1/binrel/arm-gnu-toolchain-14.3.rel1-darwin-arm64-arm-none-eabi.pkg
cat <<EOF > /tmp/ferrocene/arm-gnu-toolchain-14.3.rel1-darwin-arm64-arm-none-eabi.pkg.sha256
b93712026cec9f98a5d98dfec84e8096d32be3759642381e1982c4a5d2aa020b  /tmp/ferrocene/arm-gnu-toolchain-14.3.rel1-darwin-arm64-arm-none-eabi.pkg
EOF
sha256sum --check /tmp/ferrocene/arm-gnu-toolchain-14.3.rel1-darwin-arm64-arm-none-eabi.pkg.sha256
sudo installer -pkg /tmp/ferrocene/arm-gnu-toolchain-14.3.rel1-darwin-arm64-arm-none-eabi.pkg -target /

# There are a number of reports of very slow uploads in Mac VMs due to TSO, disable it.
# https://github.com/aws/aws-sdk/issues/469
# https://github.com/cypress-io/cypress/issues/28033#issuecomment-1879102700
# https://support.circleci.com/hc/en-us/articles/19334402064027-Troubleshooting-slow-uploads-to-S3-for-jobs-using-an-m1-macOS-resource-class
sudo sysctl net.inet.tcp.tso=0

# Free up some disk space
# Mac runners have a paltry 14gb of storage, see: https://github.com/actions/runner-images/issues/2840
if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
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

if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
    echo "::endgroup::"
fi
