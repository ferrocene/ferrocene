#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
set -xeuo pipefail

# Unlike Windows/Linux executors, Macs do not come with awscli by default
# On Mac, XCode's LLVM cannot build for WASM.
brew install awscli cmake ninja zstd llvm tidy-html5

# Needed for thumbv7em-none-eabihf & armv8r-none-eabihf cross-compilation
# brew install --cask gcc-arm-embedded
# FIXME: We worked around a problem where ARM's 13.3.rel1 package broke
# Do feel free to revert this.
brew install --cask  ferrocene/ci/formulae/gcc-arm-embedded.rb

# There are a number of reports of very slow uploads in Mac VMs due to TSO, disable it.
# https://github.com/aws/aws-sdk/issues/469
# https://github.com/cypress-io/cypress/issues/28033#issuecomment-1879102700
# https://support.circleci.com/hc/en-us/articles/19334402064027-Troubleshooting-slow-uploads-to-S3-for-jobs-using-an-m1-macOS-resource-class
sudo sysctl net.inet.tcp.tso=0
