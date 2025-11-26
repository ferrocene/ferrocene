#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
set -xeuo pipefail

# Unlike Windows/Linux executors, Macs do not come with awscli by default
# On Mac, XCode's LLVM cannot build for WASM.
brew install awscli cmake ninja zstd llvm tidy-html5

# Needed for thumbv7em-none-eabihf & armv8r-none-eabihf cross-compilation
# brew install --cask  gcc-arm-embedded
curl -Lo /tmp/gcc-arm-embedded.pkg https://developer.arm.com/-/media/Files/downloads/gnu/14.3.rel1/binrel/arm-gnu-toolchain-14.3.rel1-darwin-arm64-arm-none-eabi.pkg
sudo installer -pkg /tmp/gcc-arm-embedded.pkg -target /

# There are a number of reports of very slow uploads in Mac VMs due to TSO, disable it.
# https://github.com/aws/aws-sdk/issues/469
# https://github.com/cypress-io/cypress/issues/28033#issuecomment-1879102700
# https://support.circleci.com/hc/en-us/articles/19334402064027-Troubleshooting-slow-uploads-to-S3-for-jobs-using-an-m1-macOS-resource-class
sudo sysctl net.inet.tcp.tso=0
