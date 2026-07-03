#!/bin/bash

# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# call as run-testserver.sh <TARGET> <TEST_DEVICE_ADDR> <EMULATED> <QEMU_VERSION> <QEMU_ARCH>
set -xe

TARGET=$1
TEST_DEVICE_ADDR=$2
EMULATED=$3
QEMU_VERSION=$4
QEMU_ARCH=$5

env

EMULATOR=""
if [[ "$EMULATED" == "true" ]];
    then EMULATOR=/opt/qemu-ferrocene/${QEMU_VERSION}/bin/${QEMU_ARCH};
fi
${EMULATOR} \
/bin/test-server/$1/remote-test-server -v --bind $TEST_DEVICE_ADDR
