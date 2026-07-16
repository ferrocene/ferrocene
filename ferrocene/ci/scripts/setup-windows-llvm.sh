#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

## installing dependencies takes long on windows, so we split out
## the ones we need for building llvm into a seperate command
set -xeuo pipefail
IFS=$'\n\t'

choco install -y llvm

# Followed: https://docs.chocolatey.org/en-us/guides/create/recompile-packages/#how-to-internalizerecompile-an-existing-package-manually
# From: https://developer.arm.com/-/media/Files/downloads/gnu/15.2.rel1/binrel/arm-gnu-toolchain-15.2.rel1-mingw-w64-x86_64-arm-none-eabi.msi
aws s3 cp s3://ferrocene-ci-mirrors/manual/arm-compiler/gcc-arm-embedded.10.3.1.20251211.nupkg gcc-arm-embedded.10.3.1.20251211.nupkg
choco install gcc-arm-embedded --version="10.3.1.20251211" --source .
