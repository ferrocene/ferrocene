#!/bin/bash

# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# call as run-testserver.sh <TEST_DEVICE_ADDR> <EMULATED> <QEMU_VERSION> <QEMU_ARCH> <QEMU_CPU>
set -xe

TEST_DEVICE_ADDR=$2
EMULATED=$3
EMULATOR_VERSION=$4
EMULATOR_ARCH=$5
EMULATOR_CPU=$6

env

until [ -e /shared/testserver/remote-test-server  ]; do
  echo "Waiting until testserver binary is found in /shared/testserver/remote-test-server"
  sleep 1
done
echo "Found testserver binary, proceeding"

# Wait some in case copying takes a few seconds
sleep 5

EMULATOR=""
if [[ "$EMULATED" == "true" ]]; then
    EMULATOR=/opt/qemu-ferrocene/${EMULATOR_VERSION}/bin/${EMULATOR_ARCH};
    echo "Running under emulation"
    echo "Emulator command: ${EMULATOR}"
    echo "Emulator CPU: ${EMULATOR_CPU}"
fi

if [[ "" != "$EMULATOR_CPU" ]]; then
    export QEMU_CPU="${EMULATOR_CPU}" # we need to export the variable to make child processes use it
fi

# restart the testserver continously if it fails, but not if it exits gracefully
while true; do
    ${EMULATOR} \
    /shared/testserver/remote-test-server -v --bind $TEST_DEVICE_ADDR  && break;
done
