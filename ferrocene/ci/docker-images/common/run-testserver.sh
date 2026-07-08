#!/bin/bash

# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# call as run-testserver.sh <TARGET> <TEST_DEVICE_ADDR> <EMULATED> <QEMU_VERSION> <QEMU_ARCH> <QEMU_CPU>
set -xe

TARGET=$1
TEST_DEVICE_ADDR=$2
EMULATED=$3
EMULATOR_VERSION=$4
EMULATOR_ARCH=$5
EMULATOR_CPU=$6

env

EMULATOR=""
if [[ "$EMULATED" == "true" ]]; then
    EMULATOR=/opt/qemu-ferrocene/${EMULATOR_VERSION}/bin/${EMULATOR_ARCH};
fi

if [[ "" != "$EMULATOR_CPU" ]]; then
    export QEMU_CPU="${EMULATOR_CPU}" # we need to export the variable to make child processes use it
fi

${EMULATOR} \
/bin/test-server/$1/remote-test-server -v --bind $TEST_DEVICE_ADDR
