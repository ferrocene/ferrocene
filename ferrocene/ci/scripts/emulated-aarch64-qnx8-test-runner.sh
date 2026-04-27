#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

# this file resides in `/ferrocene/ci/scripts/(here)`
basedir="$(realpath $(dirname $0)/../../..)"
qnxdir="$(realpath $(dirname $(which qcc))/../../../../..)"
emulatordir=/tmp/emulator
nto_target=aarch64-unknown-nto-qnx800
vm_hostname=aarch64-qnx8-vm
ssh_id=id_ed25519

# actual MAC address is not important
vm_mac_addr=52:54:00:03:14:8a

# static IP configuration
# br0 has netmask 172.31.1.1/24
vm_ipv4_addr=172.31.1.11

panic() {
    echo "error:" "${@}" >&2
    exit 1
}

start_vm() {
    echo
    echo "===> setting up QEMU bridge network"
    sudo "${qnxdir}"/host/common/mkqnximage/qemu/net.sh /usr/lib/qemu/qemu-bridge-helper /etc/qemu/bridge.conf nat

    echo
    echo "===> starting QEMU"
    if [ -f "${emulatordir}"/qemu.pid ]; then
        panic a previous instance of the emulator may already be running
    fi

    ping -w1 -c1 "${vm_ipv4_addr}" && panic "node already active at configured static IP address (${vm_ipv4_addr})"


    # flags based on the QEMU invocation that `mkqnximage --arch=aarch64le --run`
    # with paths remapped to $emulatordir
    qemu-system-aarch64 \
        -machine virt-4.2 \
        -cpu cortex-a57 \
        -smp 2 \
        -m 1G \
        -drive file="${emulatordir}"/disk-qemu.vmdk,if=none,id=drv0 \
        -device virtio-blk-device,drive=drv0  \
        -netdev bridge,br=br0,id=net0 \
        -device virtio-net-device,netdev=net0,mac=${vm_mac_addr} \
        -pidfile "${emulatordir}"/qemu.pid \
        -nographic \
        -kernel "${emulatordir}"/ifs.bin \
        -serial mon:stdio \
        -object rng-random,filename=/dev/urandom,id=rng0 \
        -device virtio-rng-device,rng=rng0 \
        &> "${emulatordir}"/qemu.out &
    echo "QEMU running in background. logs at ${emulatordir}/qemu.out"

    echo
    echo "===> wait for VM's network to go online"
    ping -i3 -c4 ${vm_ipv4_addr}
}

cmd_prepare() {
    if ! [[ -d "${emulatordir}" ]]; then
        echo "error: directory ${emulatordir} does not exist"
        echo
        exit 1
    fi

    echo "===> Creating SSH key pair"
    ssh-keygen -q -t ed25519 -N '' -f "${emulatordir}"/$ssh_id

    echo
    echo "===> creating QNX image"
    tmpdir="$(mktemp -d)"
    pushd "$tmpdir"
    QNX_TARGET="${qnxdir}/target/qnx" mkqnximage \
        --noprompt \
        --hostname="${vm_hostname}" --arch=aarch64le \
        --ip="${vm_ipv4_addr}" --ssh-ident="${emulatordir}"/$ssh_id.pub

    cp output/{disk-qemu{,.vmdk},ifs.bin} "${emulatordir}/"
    popd
    rm -rf "${tmpdir}"

    echo
    echo "===> building and copying remote-test-server into emulator directory"
    stage="${REMOTE_TEST_SERVER_STAGE-1}"
    ./x build src/tools/remote-test-server --target "${nto_target}" --stage "${stage}"
    cp build/host/"stage${stage}-tools"/"${nto_target}"/release/remote-test-server "${emulatordir}"/
}

copy_to_vm() {
    scp -i "${emulatordir}"/$ssh_id -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null "$1" root@$vm_ipv4_addr:"$2"
}

on_vm() {
    ssh -i "${emulatordir}"/$ssh_id -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null root@$vm_ipv4_addr "${@}"
}

cmd_run() {
    if ! [[ -f "${emulatordir}/remote-test-server" ]]; then
        echo "error: preparation is needed before running the emulator; run:"
        echo
        echo "    $0 prepare"
        echo
        exit 1
    fi

    start_vm

    echo
    echo "===> starting remote-test-server"
    copy_to_vm "${emulatordir}"/remote-test-server /tmp/
    # networking tests in tests/ui will try to resolve 'localhost'
    on_vm sh -c 'echo "127.0.0.1 localhost" >> /etc/hosts'
    on_vm /tmp/remote-test-server -v --bind 0.0.0.0:12345 --sequential
}

if [[ $# -eq 1 ]] && [[ "$1" = "prepare" ]]; then
    cmd_prepare
elif [[ $# -eq 0 ]]; then
    cmd_run
else
    echo "usage: $0"
    echo "    or: $0 prepare"
    exit 1
fi
