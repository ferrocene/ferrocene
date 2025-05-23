# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# From the root of this repository, build with
# `docker build --tag ubuntu-24 --file ferrocene/ci/docker-images/ubuntu-24/Dockerfile .`

FROM --platform=$TARGETPLATFORM ubuntu:24.04

# As a multiplatform container we support all these: https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope
ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN <<-EOF
    set -xe
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
        gdb \
        curl \
        ca-certificates \
        python3 \
        libedit-dev \
        libssl-dev \
        zlib1g-dev \
        moreutils \
        cmake \
        python-is-python3 \
        pipx

    # On x86_64, gcc-multilib lets us compile to aarch64
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        apt-install gcc-multilib
    fi

    # Needed by the `commit-checks` CI job:
    apt-install \
        llvm-19-tools \
        llvm-19-dev \
        unzip

    # Needed to run QNX VMs
    apt-install \
        qemu-system-arm \
        qemu-system-x86 \
        bridge-utils \
        dnsmasq \
        net-tools \
        sudo

    # let regular users run sudo without password
    echo 'ALL ALL=(ALL:ALL) NOPASSWD: ALL' >> /etc/sudoers

    # Remove files that are not useful in the final image, and can be
    # regenerated by running `apt-get update`
    rm -rf /var/lib/apt/lists/*
EOF

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
    curl -Lo /tmp/awscli.zip https://awscli.amazonaws.com/awscli-exe-linux-${ARCH}-2.12.4.zip
    echo "${SHA} /tmp/awscli.zip" | sha256sum -c
    unzip -q -d /tmp/awscli /tmp/awscli.zip
    /tmp/awscli/aws/install
    rm -rf /tmp/awscli /tmp/awscli.zip
EOF

# Tell programs relying on the system language (like Python) to use UTF-8.
ENV LANG=C.UTF-8

RUN <<-EOF
    set -xe
    mkdir /home/ci
    deluser --remove-home ubuntu
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
RUN curl -LsSf https://astral.sh/uv/0.5.0/install.sh | sh
