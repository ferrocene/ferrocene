#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

# qcc resides in `somewhere/qnx710/host/linux/x86_64/usr/bin`
qnxdir="$(realpath $(dirname $(which qcc))/../../../../..)"
emulatordir=/tmp/emulator
nto_target=x86_64-pc-nto-qnx710
vm_hostname=x86_64-qnx-vm

# QNX network stack needs these to be hardcoded in the image
vm_mac_addr=52:54:00:83:26:e0
# br0 has netmask 172.31.1.1/24
vm_ipv4_addr=172.31.1.11

panic() {
    echo "error:" "${@}" >&2
    exit 1
}

on_vm() {
    echo >"${emulatordir}"/pipe.in
    echo "${@}" >"${emulatordir}"/pipe.in
    echo 'echo "===$?==="' >"${emulatordir}"/pipe.in

    while read -r line; do
        if [[ "$line" = "==="*"==="* ]]; then
            ret=$(echo "$line" | cut -d'=' -f4)

            if [ "$ret" -eq 0  ]; then
                break
            else
                panic failed to run "${@}" on VM. exit code: "$ret"
            fi
        fi

        echo "QEMU: $line"
    done <"${emulatordir}"/pipe.out
}

on_vm_nowait() {
    echo >"${emulatordir}"/pipe.in
    echo "${@}" >"${emulatordir}"/pipe.in
}

start_vm() {
    echo
    echo "===> setting up QEMU bridge network"
    sudo "${qnxdir}"/host/common/mkqnximage/qemu/net.sh /usr/lib/qemu/qemu-bridge-helper /etc/qemu/bridge.conf

    echo
    echo "===> starting QEMU"
    if [ -f "${emulatordir}"/qemu.pid ]; then
        panic a previous instance of the emulator may already be running
    fi
    rm -f "${emulatordir}"/pipe.*
    mkfifo "${emulatordir}"/pipe.in "${emulatordir}"/pipe.out
    qemu-system-x86_64 \
        -smp 2 \
        -m 1G \
        -drive file="${emulatordir}"/disk-qemu.vmdk,if=ide,id=drv0 \
        -netdev bridge,br=br0,id=net0 -device e1000,netdev=net0,mac="${vm_mac_addr}" \
        -pidfile "${emulatordir}"/qemu.pid \
        -accel kvm \
        -nographic \
        -kernel "${emulatordir}"/ifs.bin \
        -object rng-random,filename=/dev/urandom,id=rng0 \
        -device virtio-rng-pci,rng=rng0 \
        -serial pipe:"${emulatordir}"/pipe \
        -hdb fat:rw:"${emulatordir}"/shared &

    # wait until boot process completes
    while read -r line; do
        echo "QEMU: $line"

        # last boot message
        if [[ "$line" = "QNX ${vm_hostname} "*"x86pc x86_64"*  ]]; then
          break
        fi
    done <"${emulatordir}"/pipe.out
}

cmd_prepare() {
    if ! [[ -d "${emulatordir}" ]]; then
        echo "error: directory ${emulatordir} does not exist"
        echo
        exit 1
    fi

    echo
    echo "===> creating rootfs"
    tmpdir="$(mktemp -d)"
    pushd "$tmpdir"
    # NOTE `--data-size=5000 --data-inodes=40000` are required to make the test `std::fs::tests::read_large_dir` pass
    QNX_TARGET="${qnxdir}/target/qnx7" mkqnximage \
        --data-size=5000 --data-inodes=40000 --noprompt \
        --hostname="${vm_hostname}" --type=qemu --arch=x86_64 \
        --ip="${vm_ipv4_addr}" --ssh-ident=none

    # as per https://www.qnx.com/support/knowledgebase.html?id=5015Y000001gM7T
    # the ifs.build script needs to include the libpci.so.2.3 file in the IFS
    # but the stock version does not so patch it and then re-generate ifs.bin
    local ifsbuild=output/build/ifs.build
    grep 'lib/libpci.so.2.3' "${ifsbuild}" || sed -i 's|lib/libpci.so|lib/libpci.so\
lib/libpci.so.2.3|' "${ifsbuild}"
    rm output/ifs.bin
    mkifs "${ifsbuild}" output/ifs.bin

    cp output/{disk-qemu{,.vmdk},ifs.bin} "${emulatordir}/"
    popd
    rm -rf "${tmpdir}"

    echo
    echo "===> building and copying remote-test-server into rootfs"
    stage="${REMOTE_TEST_SERVER_STAGE-1}"
    ./x build src/tools/remote-test-server --target "${nto_target}" --stage "${stage}"
    mkdir -p "${emulatordir}"/shared/
    cp build/host/"stage${stage}-tools"/"${nto_target}"/release/remote-test-server "${emulatordir}"/shared/
}

cmd_run() {
    if ! [[ -f "${emulatordir}/shared/remote-test-server" ]]; then
        echo "error: preparation is needed before running the emulator; run:"
        echo
        echo "    $0 prepare"
        echo
        exit 1
    fi

    start_vm

    echo
    echo "===> starting remote-test-server"
    # `std::net` tests need this
    on_vm 'grep -q localhost /etc/hosts || echo "127.0.0.1 localhost" >> /etc/hosts'

    on_vm mount -t dos /dev/hd1t6 /mnt
    on_vm cp /mnt/remote-test-server /tmp/
    on_vm chmod +x /tmp/remote-test-server

    on_vm_nowait RUST_TEST_THREADS=1 /tmp/remote-test-server -v --bind 0.0.0.0:12345 --sequential

    while read -r line; do
        echo "QEMU: $line"
    done <"${emulatordir}"/pipe.out
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
