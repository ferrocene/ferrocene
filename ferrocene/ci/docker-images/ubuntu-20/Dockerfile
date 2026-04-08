# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# From the root of this repository, build with:
# ```
# ferrocene/ci/scripts/ensure-ubuntu-20-packages.sh
# docker build --tag ubuntu-20 --file ferrocene/ci/docker-images/ubuntu-20/Dockerfile .
# ```
# You must be authenticated with AWS in order to download its package cache.

# This Ubuntu 24.04 container is only used to grab a more recent QEMU, the real
# container is below (FROM ubuntu:20.04). This is needed because the version of
# QEMU in the Ubuntu 20.04 archives is too old, so we install QEMU on Ubuntu
# 24.04 and copy the binary over to 20.04.
#
# This actually works because the qemu-user-static package contains statically
# linked versions of QEMU, with no dynamically linked libraries. This means we
# can just copy it to a different system and it won't have any problems. Do NOT
# use the same approach for other dependencies missing in Ubuntu 20.04 unless
# you know what you're doing.
FROM --platform=$TARGETPLATFORM ubuntu:24.04 AS noble

# As a multiplatform container we support all these: https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope
ARG TARGETPLATFORM
ARG BUILDPLATFORM

# There is a mirror of the depended on QEMU version in `ferrocene-ci-mirrors` if it ever disappears.
# See https://launchpad.net/ubuntu/noble/amd64/qemu-user-static and https://launchpad.net/ubuntu/noble/arm64/qemu-user-static
COPY ferrocene/ci/docker-images/ubuntu-20/qemu-version /tmp/qemu-version
RUN <<-EOF
    set -xe
    echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections

    apt update
    apt install curl -y

    QEMU_VERSION=$(cat /tmp/qemu-version | cut -d ":" -f 2)
    echo "QEMU $QEMU_VERSION"
    rm /tmp/qemu-version
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        curl -Lo /tmp/qemu.deb http://launchpadlibrarian.net/834172758/qemu-user-static_${QEMU_VERSION}_amd64.deb
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        curl -Lo /tmp/qemu.deb http://launchpadlibrarian.net/834165403/qemu-user-static_${QEMU_VERSION}_arm64.deb
    fi
    apt install /tmp/qemu.deb -y
    rm /tmp/qemu.deb

    mkdir -p /ferrocene/packages
    dpkg -l qemu-user-static >> /ferrocene/packages/ubuntu-24.04
EOF

FROM --platform=$TARGETPLATFORM ubuntu:20.04

# As a multiplatform container we support all these: https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope
ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN <<-EOF
    set -xe
    mkdir -p /ferrocene/packages
    echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections
    alias apt-install="apt-get install -yq --no-install-recommends --option Dpkg::Options::=--force-confnew"

    apt-get update
    # Update all the dependencies
    apt-get upgrade -y

    # Install required packages
    apt-install \
        git \
        build-essential \
        ninja-build \
        openssh-client \
        file \
        curl \
        ca-certificates \
        libedit-dev \
        libssl-dev \
        zlib1g-dev \
        moreutils \
        zstd \
        sudo \
        gcc-10 \
        g++-10 \
        gcc-s390x-linux-gnu \
        g++-s390x-linux-gnu \
        gcc-powerpc64le-linux-gnu \
        g++-powerpc64le-linux-gnu

    # Needed to support the QEMU copied over from Ubuntu 24.04 (see above).
    # Replace this with qemu-user-static once we're not on Ubuntu 20.04.
    apt-install binfmt-support

    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        # Needed for aarch64-unknown-linux-gnu cross-compilation
        apt-install \
            g++-aarch64-linux-gnu \
            gcc-aarch64-linux-gnu \
            g++-10-aarch64-linux-gnu \
            gcc-10-aarch64-linux-gnu \
            binutils-aarch64-linux-gnu \
            libc6-dev-arm64-cross
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        # Needed for x86_64-unknown-linux-gnu cross-compilation
        apt-install \
            g++-x86-64-linux-gnu \
            gcc-x86-64-linux-gnu \
            g++-10-x86-64-linux-gnu \
            gcc-10-x86-64-linux-gnu \
            binutils-x86-64-linux-gnu \
            libc6-dev-amd64-cross
    fi

    # make gcc 10 default
    sudo update-alternatives \
      --install /usr/bin/gcc gcc /usr/bin/gcc-10 100 \
      --slave /usr/bin/g++ g++ /usr/bin/g++-10 \
      --slave /usr/bin/gcov gcov /usr/bin/gcov-10 \
      --slave /usr/bin/aarch64-linux-gnu-gcc aarch64-linux-gnu-gcc /usr/bin/aarch64-linux-gnu-gcc-10 \
      --slave /usr/bin/aarch64-linux-gnu-g++ aarch64-linux-gnu-g++ /usr/bin/aarch64-linux-gnu-g++-10

    # Needed for thumbv7em-none-eabihf & armv8r-none-eabihf cross-compilation
    # no gcc 10 package exists for this, this is an even older gcc than the standard gcc (9.2 vs 9.4)
    apt-install gcc-arm-none-eabi

    # Needed for the wasm32-unknown-unknown target
    apt-install clang
    # Needed to install AWS CLI during the build:
    apt-install unzip

    # Needed to install git during the build:
    apt-install \
        dh-autoreconf \
        libcurl4-gnutls-dev \
        libexpat1-dev \
        gettext \
        libz-dev \
        libssl-dev

    # Remove files that are not useful in the final image, and can be
    # regenerated by running `apt-get update`
    rm -rf /var/lib/apt/lists/*
EOF

# Install a version of the RISC-V 64 bit toolchain that is modern, as the packaged ones on Ubuntu 20.04
# are too old to be useful (they don't know about the `z` ISA flag).
#
# We get x86_64 Linux binaries from https://www.embecosm.com/resources/tool-chain-downloads/#riscv-linux
#
# These builds are unfortunately only for x86_64. To add new targets, we'll need to use a newer Ubuntu
# image, or build our own from source via https://github.com/riscv-collab/riscv-gnu-toolchain.
RUN <<-EOF
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        mkdir /opt/riscv-toolchain
        PACKAGE_URL="https://buildbot.embecosm.com/job/riscv64-linux-gcc-ubuntu2004/20/artifact/riscv64-embecosm-linux-gcc-ubuntu2004-20240407.tar.gz"
        SHA="ddebec169a6c3a29b6cd0133f54f93c8f7addab9b3b5fb5366fa73dda834d7b7"
        curl -Lo /tmp/riscv-toolchain.tar.gz $PACKAGE_URL
        echo "$SHA /tmp/riscv-toolchain.tar.gz" | sha256sum -c
        tar xf /tmp/riscv-toolchain.tar.gz -C /opt/riscv-toolchain --strip-components=1
        rm /tmp/riscv-toolchain.tar.gz
        /opt/riscv-toolchain/bin/riscv64-unknown-linux-gnu-gcc --version

        echo "$SHA $PACKAGE_URL" >> /ferrocene/packages/downloads
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        echo "riscv64gc-unknown-linux-gnu toolchain setup skipped on aarch64-unknown-linux-gnu host."
    else
        echo "Unsupported platform"
        exit 1
    fi
EOF
ENV PATH "/opt/riscv-toolchain/bin:$PATH"

# Grab the newer QEMU from Ubuntu 24.04
# This requires the binfmt-support package, which is normally a dependency of
# qemu-user-static.
COPY --from=noble /usr/bin/qemu-aarch64-static /usr/bin/qemu-aarch64-static
COPY --from=noble /usr/libexec/qemu-binfmt/aarch64-binfmt-P /usr/libexec/qemu-binfmt/aarch64-binfmt-P
COPY --from=noble /usr/bin/qemu-arm-static /usr/bin/qemu-arm-static
COPY --from=noble /usr/libexec/qemu-binfmt/arm-binfmt-P /usr/libexec/qemu-binfmt/arm-binfmt-P
# This was removed in 24.04 and is a copy of the 22.04 file.
COPY ferrocene/ci/docker-images/ubuntu-20/binfmt-qemu-aarch64 /usr/share/binfmts/qemu-aarch64
COPY ferrocene/ci/docker-images/ubuntu-20/binfmt-qemu-arm /usr/share/binfmts/qemu-arm

# Install a recent NodeJS version, as the one shipped in Ubuntu 20.04 is too
# old to run the rustdoc-js test suite. Note that we cannot install the most
# recent NodeJS LTS release, as it's built with a more recent glibc version
# than the one shipped with Ubuntu 20.04.
#
# If you are reading this comment because you need to upgrade past NodeJS 16,
# you will unfortunately need to change this to build NodeJS from source.
RUN <<-EOF
    set -xe
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        ARCH="x64"
        SHA="874463523f26ed528634580247f403d200ba17a31adf2de98a7b124c6eb33d87"
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        ARCH="arm64"
        SHA="e88d86154d1ce53dc52fd74d79d4bfdf0b05f58c0bb2639adfa36e9378b770c4"
    else
        echo "Unsupported platform"
        exit 1
    fi
    mkdir /opt/nodejs
    PACKAGE_URL="https://nodejs.org/dist/v16.20.2/node-v16.20.2-linux-${ARCH}.tar.xz"
    curl -Lo /tmp/nodejs.tar.xz $PACKAGE_URL
    echo "${SHA} /tmp/nodejs.tar.xz" | sha256sum -c
    tar xf /tmp/nodejs.tar.xz -C /opt/nodejs --strip-components=1
    rm /tmp/nodejs.tar.xz
    /opt/nodejs/bin/node --version

    echo "$SHA $PACKAGE_URL" >> /ferrocene/packages/downloads
EOF
ENV PATH "/opt/nodejs/bin:$PATH"

# Install AWS CLI from the published binaries, as no source tarball is provided
# and the version in the Ubuntu 20.04 archive is too old and doesn't support
# `aws ecr get-login-password`.
# Install AWS CLI from the published binaries, as no source tarball is provided
# and the version in the Ubuntu 24.04 is a snap.
RUN <<-EOF
    set -xe
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        ARCH="x86_64"
        SHA="0556162145e68889a27f835307b9dad56de60fda4256d2e246e9699f24535a08"
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        ARCH="aarch64"
        SHA="33b1c2a9daab91ae458516e934b1532e2174643aea2ccfa81ab8c5e9d4ed381b"
    else
        echo "Unsupported platform"
        exit 1
    fi
    PACKAGE_URL="https://awscli.amazonaws.com/awscli-exe-linux-${ARCH}-2.12.4.zip"
    curl -Lo /tmp/awscli.zip $PACKAGE_URL
    echo "${SHA} /tmp/awscli.zip" | sha256sum -c
    unzip -q -d /tmp/awscli /tmp/awscli.zip
    /tmp/awscli/aws/install
    rm -rf /tmp/awscli /tmp/awscli.zip

    echo "$SHA $PACKAGE_URL" >> /ferrocene/packages/downloads
EOF

# Install pre-built packages for Ubuntu 20.
# See ferrocene/ci/mirrors/README.md.
COPY ferrocene/ci/mirrors/ ./mirrors/
RUN cd mirrors && sha256sum --ignore-missing --strict --check hashes.txt
RUN <<-EOF
    set -eu

    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        host_platform=x86_64
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        host_platform=aarch64
    fi
    for tarball in \
        musl-cross-make-${host_platform}-to-aarch64 \
        musl-cross-make-${host_platform}-to-x86_64 \
        cmake-3.21.1-${host_platform} \
        python-3.12.3-${host_platform} \
        gdb-12.1-${host_platform}
    do
        filename=mirrors/${tarball}.tar.xz
        echo "extracting $tarball"
        tar -C /usr/local -xf "$filename"
        rm "$filename"
    done
EOF

# To run chroot inside the container we need sudo access. Copy a configuration
# file allowing all users inside the container to use sudo.
COPY ferrocene/ci/docker-images/ubuntu-20/sudoers /etc/sudoers

# Tell programs relying on the system language (like Python) to use UTF-8.
ENV LANG=C.UTF-8

# Automatic compiler detection doesn't work for some targets.
ENV CC_aarch64_unknown_none=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_none=aarch64-linux-gnu-g++
ENV CC_aarch64_unknown_ferrocene.facade=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_ferrocene.facade=aarch64-linux-gnu-g++
ENV CC_armv8r_none_eabihf=arm-none-eabi-gcc
ENV CXX_armv8r_none_eabihf=arm-none-eabi-g++
ENV CFLAGS_aarch64_unknown_nto_qnx710=-Vgcc_ntoaarch64le_cxx
ENV CFLAGS_x86_64_pc_nto_qnx710=-Vgcc_ntox86_64_cxx
ENV CC_riscv64gc_unknown_linux_gnu=riscv64-unknown-linux-gnu-gcc
ENV CXX_riscv64gc_unknown_linux_gnu=riscv64-unknown-linux-gnu-g++

# Automatic CFLAGs do not work for some targets.
ENV CFLAGS_armv8r_none_eabihf="-march=armv8-r -mfpu=fp-armv8"
# FIXME replace 'armv8-a' with 'armv8-r' when GCC version is bumped to 11+
ENV CFLAGS_aarch64v8r_unknown_none_softfloat="-mstrict-align -march=armv8-a+nofp+nosimd"
ENV CFLAGS_aarch64r82_unknown_none_softfloat="-mstrict-align -march=armv8-a+nofp+nosimd"

COPY --from=noble /ferrocene/packages/ubuntu-24.04 /ferrocene/packages/ubuntu-24.04
RUN <<-EOF
    set -xe
    dpkg -l >> /ferrocene/packages/ubuntu-20.04
EOF


RUN <<-EOF
    set -xe
    mkdir /home/ci
    addgroup --gid 1000 ci
    addgroup --gid 1001 ci-usergroup
    adduser --home /home/ci --uid 1000 --gid 1000 --gecos "" --disabled-password ci
    adduser --uid 1001 --gid 1001 --gecos "" --disabled-password ci-user
    chown -R ci: /home/ci
EOF
USER ci
WORKDIR /home/ci

# We prefer uv over other solutions, it's in Rust and fast
ENV PATH="/home/ci/.venv/bin:/home/ci/.local/bin:$PATH"
RUN curl -LsSf https://astral.sh/uv/0.9.26/install.sh | sh

# Tool needed to generate SBOMs
RUN curl -sSfL https://get.anchore.io/syft | sh -s -- -b /home/ci/.local/bin v1.41.1
