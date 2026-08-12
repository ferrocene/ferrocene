# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# From the root of this repository, build with:
# ```
# ferrocene/ci/scripts/ensure-ubuntu-20-packages.sh
# docker build --tag runner-glibc-2.17 --file ferrocene/ci/docker-images/runner-glibc-2.17/Dockerfile .
# ```
# You must be authenticated with AWS in order to download its package cache.

# Increment this to force a rebuild in CI, even if you no other changes
# Generation: 1

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

FROM --platform=$TARGETPLATFORM ghcr.io/rust-lang/centos:7 AS builder

# As a multiplatform container we support all these: https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope
ARG TARGETPLATFORM
ARG BUILDPLATFORM

# CentOS 7 EOL is June 30, 2024, but the repos remain in the vault.
RUN sed -i /etc/yum.repos.d/*.repo -e 's!^mirrorlist!#mirrorlist!' \
  -e 's!^#baseurl=http://mirror.centos.org/!baseurl=https://vault.centos.org/!'
RUN sed -i 's/enabled=1/enabled=0/' /etc/yum/pluginconf.d/fastestmirror.conf


RUN mkdir -p /ferrocene/packages
RUN yum -y install \
    wget curl \
    zstd xz bzip2 gzip unzip \
    tar \
    make patch \
    file which \
    gawk \
    openssh-clients \
    ca-certificates \
    sudo \
    binutils \
    procps-ng \
    perl-core \
    shadow-utils

# See ferrocene/ci/mirrors/README.md.
COPY ferrocene/ci/mirrors/ ./mirrors/
# RUN cd mirrors && sha256sum --ignore-missing --strict --check hashes.txt
RUN <<-EOF
    set -eu

    mkdir -p /ferrocene-buildroot
    mkdir -p /stage/coreutils

    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        host_platform=x86_64
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        host_platform=aarch64
    fi

    for LINE in $(cat /mirrors/hashes.txt | tr -s " " | cut -f 2 -d " "); do
        package_file=$(echo "${LINE}" | cut -f 2)
        # we know that the host arch of the package is always the first occurence, the
        # target arch is optional and only occurs for the musl-cross package
        host_arch=$(echo $package_file | awk '{ match($0, /(aarch64|x86_64)/); print substr($0, RSTART, RLENGTH);}' )

        if [ "$host_platform" = "$host_arch" ]; then
            echo "Installing package ${package_file}"
            if [[ $package_file =~ "coreutils" ]];  then
                # We can't later run "ADD" for coreutils since it's arch dependent so we stage it
                # and copy from the stage
                echo "Installing coreutils to /stage/coreutils"
                tar -C /stage/coreutils -xf "mirrors/$package_file"
            elif [[ $package_file =~ "ferrocene-buildroot" ]];  then
                echo "Installing buildroot to /ferrocene-buildroot"
                tar -C /ferrocene-buildroot -xf "mirrors/$package_file"
            else
                tar -C /usr/local -xf "mirrors/$package_file"
            fi
        else
            echo "Skipping package ${package_file}, wrong host arch (${host_arch})"
        fi
    done

    # TESTING CHANGE: Our configuration expects the compiler to be named
    # aarch64-linux-gnu-gcc
    cd /ferrocene-buildroot/bin
    ln -s aarch64-unknown-linux-gnu-gcc aarch64-linux-gnu-gcc && true
    ln -s aarch64-unknown-linux-gnu-c++ aarch64-linux-gnu-c++ && true
EOF

RUN <<-EOF
    set -eu

    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        host_platform=x86_64
        curl -Lo nina-build.zip https://github.com/ninja-build/ninja/releases/download/v1.12.1/ninja-linux.zip
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        host_platform=aarch64
        curl -Lo nina-build.zip https://github.com/ninja-build/ninja/releases/download/v1.12.1/ninja-linux-aarch64.zip
    fi
    unzip nina-build.zip
    mv ninja /usr/local/bin
EOF

# ensure that "cc" exists as a symlink, use --force for now as newer buildroots
# already have the symlink
RUN cd /ferrocene-buildroot/bin && ln --force -s gcc cc

FROM --platform=$TARGETPLATFORM ghcr.io/rust-lang/centos:7 AS base

# As a multiplatform container we support all these: https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope
ARG TARGETPLATFORM
ARG BUILDPLATFORM

# CentOS 7 EOL is June 30, 2024, but the repos remain in the vault.
RUN sed -i /etc/yum.repos.d/*.repo -e 's!^mirrorlist!#mirrorlist!' \
  -e 's!^#baseurl=http://mirror.centos.org/!baseurl=https://vault.centos.org/!'
RUN sed -i 's/enabled=1/enabled=0/' /etc/yum/pluginconf.d/fastestmirror.conf


RUN mkdir -p /ferrocene/packages
RUN yum -y install \
    wget curl \
    zstd xz bzip2 gzip unzip \
    zlib-devel \
    glibc-devel glibc-devel.i686 \
    glibc-static glibc-static.i686 \
    tar \
    make patch \
    file which \
    gawk \
    openssh-clients \
    ca-certificates \
    libedit-devel \
    sudo \
    binutils \
    procps-ng \
    perl-core \
    shadow-utils

# Install a version of the RISC-V 64 bit toolchain that is modern, as the packaged ones on Ubuntu 20.04
# are too old to be useful (they don't know about the `z` ISA flag).
#
# We get x86_64 Linux binaries from https://www.embecosm.com/resources/tool-chain-downloads/#riscv-linux
#
# These builds are unfortunately only for x86_64. To add new targets, we'll need to use a newer Ubuntu
# image, or build our own from source via https://github.com/riscv-collab/riscv-gnu-toolchain.
# RUN <<-EOF
#     if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
#         mkdir /opt/riscv-toolchain
#         PACKAGE_URL="https://buildbot.embecosm.com/job/riscv64-linux-gcc-ubuntu2004/20/artifact/riscv64-embecosm-linux-gcc-ubuntu2004-20240407.tar.gz"
#         SHA="ddebec169a6c3a29b6cd0133f54f93c8f7addab9b3b5fb5366fa73dda834d7b7"
#         curl -Lo /tmp/riscv-toolchain.tar.gz $PACKAGE_URL
#         echo "$SHA /tmp/riscv-toolchain.tar.gz" | sha256sum -c
#         tar xf /tmp/riscv-toolchain.tar.gz -C /opt/riscv-toolchain --strip-components=1
#         rm /tmp/riscv-toolchain.tar.gz
#         /opt/riscv-toolchain/bin/riscv64-unknown-linux-gnu-gcc --version

#         echo "$SHA $PACKAGE_URL" >> /ferrocene/packages/downloads
#     elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
#         echo "riscv64gc-unknown-linux-gnu toolchain setup skipped on aarch64-unknown-linux-gnu host."
#     else
#         echo "Unsupported platform"
#         exit 1
#     fi
# EOF
ENV PATH "/opt/riscv-toolchain/ubuntu2004-20240407/bin:$PATH"


# Install a recent NodeJS version, as the one shipped in Ubuntu 20.04 is too
# old to run the rustdoc-js test suite. Note that we cannot install the most
# recent NodeJS LTS release, as it's built with a more recent glibc version
# than the one shipped with Ubuntu 20.04.
#
# If you are reading this comment because you need to upgrade past NodeJS 16,
# you will unfortunately need to change this to build NodeJS from source.
# RUN <<-EOF
#     set -xe
#     if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
#         ARCH="x64"
#         SHA="874463523f26ed528634580247f403d200ba17a31adf2de98a7b124c6eb33d87"
#     elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
#         ARCH="arm64"
#         SHA="e88d86154d1ce53dc52fd74d79d4bfdf0b05f58c0bb2639adfa36e9378b770c4"
#     else
#         echo "Unsupported platform"
#         exit 1
#     fi
#     mkdir /opt/nodejs
#     PACKAGE_URL="https://nodejs.org/dist/v16.20.2/node-v16.20.2-linux-${ARCH}.tar.xz"
#     curl -Lo /tmp/nodejs.tar.xz $PACKAGE_URL
#     echo "${SHA} /tmp/nodejs.tar.xz" | sha256sum -c
#     tar xf /tmp/nodejs.tar.xz -C /opt/nodejs --strip-components=1
#     rm /tmp/nodejs.tar.xz
#     /opt/nodejs/bin/node --version

#     echo "$SHA $PACKAGE_URL" >> /ferrocene/packages/downloads
# EOF
ENV PATH "/opt/nodejs/16.20.2/bin:$PATH"

# Install AWS CLI from the published binaries, as no source tarball is provided
# and the version provided by packages is too old and doesn't support
# `aws ecr get-login-password`.
# Install AWS CLI from the published binaries, as no source tarball is provided
COPY --chmod=0700 ferrocene/ci/docker-images/common/setup-awscli.sh /tmp/setup-awscli.sh
COPY --chmod=0600 ferrocene/ci/awscli-version /tmp/awscli-version
RUN /tmp/setup-awscli.sh


COPY --from=builder /ferrocene-buildroot /ferrocene-buildroot
COPY --from=builder /usr/local /usr/local
COPY --from=builder stage/coreutils/ /usr/

ENV PATH=/ferrocene-buildroot/bin/:/usr/local/bin/:/usr/local/cmake-3.31.12-linux-x86_64/bin/:/usr/local/cmake-3.31.12-linux-aarch64/bin/:$PATH

ENV PKG_CONFIG_PATH=/usr/local/lib64/pkgconfig/

# add /usr/local/lib to ld's path
RUN echo '/usr/local/lib/' > /etc/ld.so.conf.d/openssl.conf
RUN echo '/usr/local/lib64' >> /etc/ld.so.conf.d/openssl.conf
RUN echo '/usr/local/lib/' >> /etc/ld.so.conf.d/usr-local.conf
RUN echo '/usr/local/lib64/' >> /etc/ld.so.conf.d/usr-local.conf
RUN echo '/ferrocene-buildroot/lib64/' >> /etc/ld.so.conf.d/ferrocene-buildroot.conf
RUN ldconfig

# To run chroot inside the container we need sudo access. Copy a configuration
# file allowing all users inside the container to use sudo.
COPY ferrocene/ci/docker-images/common/sudoers /etc/sudoers

# Tell programs relying on the system language (like Python) to use UTF-8.
ENV LANG=en_US.utf8

# Automatic compiler detection doesn't work for some targets.


ENV CC=clang
ENV CXX=clang++

ENV RUST_CONFIGURE_ARGS="--set target.x86_64-unknown-linux-gnu.linker=clang \
      --set target.x86_64-unknown-linux-gnu.ar=/ferrocene-buildroot/bin/llvm-ar \
      --set target.x86_64-unknown-linux-gnu.ranlib=/ferrocene-buildroot/bin/llvm-ranlib \
      --set target.aarch64-unknown-linux-gnu.linker=clang \
      --set target.aarch64-unknown-linux-gnu.ar=/ferrocene-buildroot/bin/llvm-ar \
      --set target.aarch64-unknown-linux-gnu.ranlib=/ferrocene-buildroot/bin/llvm-ranlib \
      --set rust.bootstrap-override-lld=true \
      --set rust.lto=thin \
      --set rust.codegen-units=1"

ENV CC_aarch64_unknown_none=aarch64-unknown-linux-gnu-gcc
ENV CXX_aarch64_unknown_none=aarch64-unknown-linux-gnu-g++
ENV CC_aarch64_unknown_ferrocene.facade=aarch64-unknown-linux-gnu-gcc
ENV CXX_aarch64_unknown_ferrocene.facade=aarch64-unknown-linux-gnu-g++
ENV CC_armv8r_none_eabihf=arm-none-eabi-gcc
ENV CXX_armv8r_none_eabihf=arm-none-eabi-g++
ENV CFLAGS_aarch64_unknown_nto_qnx710=-Vgcc_ntoaarch64le_cxx
ENV CFLAGS_x86_64_pc_nto_qnx710=-Vgcc_ntox86_64_cxx
ENV CFLAGS_aarch64_unknown_qnx=-Vgcc_ntoaarch64le_cxx
ENV CC_aarch64_unknown_qnx=qcc
ENV CFLAGS_x86_64_pc_qnx=-Vgcc_ntox86_64_cxx
ENV CC_x86_64_pc_qnx=qcc


ENV CC_riscv64gc_unknown_linux_gnu=riscv64-unknown-linux-gnu-gcc
ENV CXX_riscv64gc_unknown_linux_gnu=riscv64-unknown-linux-gnu-g++
ENV CPATH_riscv64gc_unknown_linux_gnu=

ENV CC_aarch64_unknown_linux_musl=aarch64-linux-musl-gcc
ENV CXX_aarch64_unknown_linux_musl=aarch64-linux-musl-g++
ENV CPATH_aarch64_unknown_linux_musl=/usr/local/aarch64-linux-musl/include/

ENV CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc
ENV CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
ENV CPATH_x86_64_unknown_linux_musl=/usr/local/x86_64-linux-musl/include/

ENV CC_s390x_unknown_linux_gnu=s390x-linux-gnu-gcc
ENV CXX_s390x_unknown_linux_gnu=s390x-linux-gnu-g++

# Automatic CFLAGs do not work for some targets.
ENV CFLAGS_armv8r_none_eabihf="-march=armv8-r -mfpu=fp-armv8"
# FIXME replace 'armv8-a' with 'armv8-r' when GCC version is bumped to 11+
ENV CFLAGS_aarch64v8r_unknown_none_softfloat="-mstrict-align -march=armv8-a+nofp+nosimd"
ENV CFLAGS_aarch64r82_unknown_none_softfloat="-mstrict-align -march=armv8-a+nofp+nosimd"

RUN <<-EOF
    set -xe

    groupadd --gid 1000 ci
    groupadd --gid 1002 circleci

    useradd --uid 1000 --gid 1000 -d /home/ci -m ci
    useradd --uid 1001 --gid 1002 -d /home/circleci circleci
EOF

USER ci
WORKDIR /project

# We prefer uv over other solutions, it's in Rust and fast
ENV PATH="/home/ci/.venv/bin:/home/ci/.local/bin:$PATH"
RUN curl -LsSf https://astral.sh/uv/0.9.26/install.sh | sh

# Tool needed to generate SBOMs
RUN curl -sSfL https://get.anchore.io/syft | sh -s -- -b /home/ci/.local/bin v1.41.1
