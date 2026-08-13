# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# From the root of this repository, build with:
# ```
# ferrocene/ci/scripts/ensure-ubuntu-20-packages.sh
# docker build --tag qnx-runner --file ferrocene/ci/docker-images/qnx-runner/Dockerfile .
# ```
# You must be authenticated with AWS in order to download its package cache.

# Increment this to force a rebuild in CI, even if you no other changes
# Generation: 1

FROM --platform=$TARGETPLATFORM ubuntu:20.04 AS builder
# As a multiplatform container we support all these: https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope
ARG TARGETPLATFORM
ARG BUILDPLATFORM

# See ferrocene/ci/mirrors/README.md.
COPY ferrocene/ci/mirrors/ ./mirrors/
# RUN cd mirrors && sha256sum --ignore-missing --strict --check hashes.txt
SHELL ["/bin/bash", "-c"]
RUN <<-EOF

    set -xeu
    mkdir -p /ferrocene/packages
    mkdir -p /stage/coreutils

    echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections

    apt-get update
    # Update all the dependencies
    apt-get upgrade -y

    # Install required packages
    # The alias trick seems to be problematic with using bash as a shell, so
    # have the apt-get options here directly
    apt-get install -yq --no-install-recommends --option Dpkg::Options::=--force-confnew \
        gpg \
        gpg-agent \
        git \
        openssh-client \
        file \
        curl \
        ca-certificates \
        zlib1g-dev \
        moreutils \
        zstd \
        xz-utils \
        unzip \
        sudo

    mkdir -p /ferrocene-buildroot

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
        gpg \
        gpg-agent \
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
        xz-utils \
        unzip \
        sudo \
        gcc-10 \
        g++-10 \
        gcc-multilib

    # make gcc 10 default
    sudo update-alternatives \
      --install /usr/bin/gcc gcc /usr/bin/gcc-10 100 \
      --slave /usr/bin/g++ g++ /usr/bin/g++-10 \
      --slave /usr/bin/gcov gcov /usr/bin/gcov-10 \
      --slave /usr/bin/aarch64-linux-gnu-gcc aarch64-linux-gnu-gcc /usr/bin/aarch64-linux-gnu-gcc-10 \
      --slave /usr/bin/aarch64-linux-gnu-g++ aarch64-linux-gnu-g++ /usr/bin/aarch64-linux-gnu-g++-10

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


# Install a recent NodeJS version, as the one shipped in Ubuntu 20.04 is too
# old to run the rustdoc-js test suite. Note that we cannot install the most
# recent NodeJS LTS release, as it's built with a more recent glibc version
# than the one shipped with Ubuntu 20.04.
#
# If you are reading this comment because you need to upgrade past NodeJS 16,
# you will unfortunately need to change this to build NodeJS from source.

ENV PATH "/opt/nodejs/16.20.2/bin:$PATH"

# Install AWS CLI from the published binaries, as no source tarball is provided
# and the version in the Ubuntu 20.04 archive is too old and doesn't support
# `aws ecr get-login-password`.
# Install AWS CLI from the published binaries, as no source tarball is provided
# and the version in the Ubuntu 24.04 is a snap.
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

# Ubuntu and CentOS don't agree on the location of the CA certs and our git
# inherits the setting from the system it was built on
RUN git config --system http.sslcainfo /etc/ssl/certs/ca-certificates.crt

# To run chroot inside the container we need sudo access. Copy a configuration
# file allowing all users inside the container to use sudo.
COPY ferrocene/ci/docker-images/common/sudoers /etc/sudoers

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
ENV CFLAGS_aarch64_unknown_qnx=-Vgcc_ntoaarch64le_cxx
ENV CC_aarch64_unknown_qnx=qcc
ENV CFLAGS_x86_64_pc_qnx=-Vgcc_ntox86_64_cxx
ENV CC_x86_64_pc_qnx=qcc
ENV CC_riscv64gc_unknown_linux_gnu=riscv64-unknown-linux-gnu-gcc
ENV CXX_riscv64gc_unknown_linux_gnu=riscv64-unknown-linux-gnu-g++
ENV CC_aarch64_unknown_linux_musl=aarch64-linux-musl-gcc
ENV CXX_aarch64_unknown_linux_musl=aarch64-linux-musl-g++

ENV CPATH="/ferrocene-buildroot/include:/usr/local/aarch64-linux-musl/include/:/usr/local/x86_64-linux-musl/include/:/usr/local/include:/usr/include"

# Automatic CFLAGs do not work for some targets.
ENV CFLAGS_armv8r_none_eabihf="-march=armv8-r -mfpu=fp-armv8"
# FIXME replace 'armv8-a' with 'armv8-r' when GCC version is bumped to 11+
ENV CFLAGS_aarch64v8r_unknown_none_softfloat="-mstrict-align -march=armv8-a+nofp+nosimd"
ENV CFLAGS_aarch64r82_unknown_none_softfloat="-mstrict-align -march=armv8-a+nofp+nosimd"

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