#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

echo "CPU"
echo "==="
lscpu

echo
echo "System memory"
echo "============="
free -h

echo "Disk"
echo "===="
df -h
