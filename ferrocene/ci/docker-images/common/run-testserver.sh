#!/bin/bash

# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# call as run-testserver.sh <TARGET> <TEST_DEVICE_ADDR> <EMULATED> <QEMU_VERSION> <QEMU_ARCH> <QEMU_CPU>
set -xe

TARGET=$1
TEST_DEVICE_ADDR=$2
EMULATED=$3
QEMU_VERSION=$4
QEMU_ARCH=$5
QEMU_CPU=$6

env

EMULATOR=""
if [[ "$EMULATED" == "true" ]]; then
    EMULATOR=/opt/qemu-ferrocene/${QEMU_VERSION}/bin/${QEMU_ARCH};
fi

if [[ "" == $QEMU_CPU ]]; then
    unset QEMU_CPU # qemu barfs if this is set to "", so we'll make sure to unset it.
fi

${EMULATOR} \
/bin/test-server/$1/remote-test-server -v --bind $TEST_DEVICE_ADDR
