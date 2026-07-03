#!/bin/bash
set -xe

EMULATOR=""
if $EMULATED; then EMULATOR=/opt/qemu-ferrocene/${QEMU_VERSION}/bin/${QEMU_ARCH}; fi
${EMULATOR} \
/bin/test-server/$1/remote-test-server -v --bind $TEST_DEVICE_ADDR
