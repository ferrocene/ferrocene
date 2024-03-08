#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

echo "CPU"
echo "==="
if [[ "${OSTYPE}" =~ ^darwin.* ]]; then
    sysctl -n machdep.cpu.brand_string
else if [[ "${OSTYPE}" != "msys" ]]; then
    lscpu
fi

echo
echo "System memory"
echo "============="
if [[ "${OSTYPE}" =~ ^darwin.* ]]; then
    vm_stat
else if [[ "${OSTYPE}" != "msys" ]]; then
    free -h
fi

echo "Disk"
echo "===="
df -h
