#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
set -xeo pipefail

# Ensure we never get asked/prompted, always take the new config
echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections

apt install -y wajig

wajig sizes | tail --lines 30
du --human-readable / | sort --human-numeric-sort | tail --lines 100

