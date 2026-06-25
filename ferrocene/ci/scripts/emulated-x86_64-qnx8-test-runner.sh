#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

. "$(dirname $0)"/qnx-common.sh

nto_target=x86_64-pc-qnx
vm_hostname=x86_64-qnx8-vm

start_vm() {
    qnx8_set_up_bridge_network

    echo
    echo "===> starting QEMU"
    check_no_other_emulator_is_running
    check_ip_address_is_free

    # flags based on the QEMU invocation that `mkqnximage --run` does
    # with paths remapped to $emulatordir
    # `mkqnximage --run` enables KVM which we can't use in CI. instead,
    # use `-cpu max` as the startup library requires CPU features that
    # are not enabled by default. without `-cpu max` QEMU will hang
    # set memory to 11G as `tests/ui/codegen/huge-stacks.rs` requires 5GB+ and has two
    # variants which can run in parallel
    qemu-system-x86_64 \
        -smp 2 \
        -cpu max \
        -m 11G \
        -drive file="${emulatordir}"/disk-qemu.vmdk,if=ide,id=drv0 \
        -netdev bridge,br=br0,id=net0 -device virtio-net-pci,netdev=net0 \
        -pidfile "${emulatordir}"/qemu.pid \
        -nographic \
        -kernel "${emulatordir}"/ifs.bin \
        -object rng-random,filename=/dev/urandom,id=rng0 -device virtio-rng-pci,rng=rng0 \
        -serial mon:stdio
}

cmd_prepare() {
    check_emulatordir_exists

    echo
    echo "===> building remote-test-server"
    ./x build src/tools/remote-test-server --target "${nto_target}"

    echo
    echo "===> creating initial IFS"
    tmpdir="$(mktemp -d)"
    pushd "$tmpdir"
    # NOTE `--data-size=5000 --data-inodes=40000` are required to make the test `std::fs::tests::read_large_dir` pass
    QNX_TARGET="${qnxdir}/target/qnx" mkqnximage \
        --data-size=5000 --data-inodes=40000 --noprompt \
        --hostname="${vm_hostname}" --type=qemu --arch=x86_64 \
        --ip="${vm_ipv4_addr}" --ssh-ident=none

    echo
    echo "===> re-building a custom IFS that includes remote-test-server"

    # add remote-test-servere binary to IFS
    local ifsbuild=output/build/ifs.build
    cp "$basedir"/build/host/"stage2-tools"/"${nto_target}"/release/remote-test-server "${tmpdir}"/output/build/
    echo 'remote-test-server=output/build/remote-test-server' >> "${ifsbuild}"

    # run remote-test-server once startup is complete
    local startup=output/build/startup.sh
    # UI tests and libstd tests will try to resolve 'localhost'
    echo 'grep -q localhost /etc/hosts || echo "127.0.0.1 localhost" >> /etc/hosts' >> "${startup}"
    # the default `TMPDIR` (`/tmp`) uses the `shmem` filesystem which has a
    # number of limitations around symlink support and the creation of
    # directories. use a directory in `/data` (QNX6 FS) as `TMPDIR` to avoid
    # failing tests due to FS limitations
    echo 'mkdir -p /data/tmp' >> "${startup}"
    echo 'RUST_TEST_THREADS=1 TMPDIR=/data/tmp remote-test-server -v --bind 0.0.0.0:12345 --sequential' >> "${startup}"

    rm output/ifs.bin
    mkifs "${ifsbuild}" output/ifs.bin

    cp output/{disk-qemu{,.vmdk},ifs.bin} "${emulatordir}/"
    popd
    rm -rf "${tmpdir}"
}

cmd_run() {
    if ! [[ -f "${emulatordir}/ifs.bin" ]]; then
        echo "error: preparation is needed before running the emulator; run:"
        echo
        echo "    $0 prepare"
        echo
        exit 1
    fi

    start_vm
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
