# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM centos7-base AS build
ARG TARGETPLATFORM

RUN yum -y install \
    binutils \
    flex bison

COPY gcc-11.tar.xz /gcc-11.tar.xz
RUN mkdir -p /opt/local/gcc-11

RUN tar -xf /gcc-11.tar.xz -C /opt/local/gcc-11
ENV PATH=/opt/local/gcc-11/bin:$PATH

COPY gcc-with-dependencies.tar.xz /gcc-with-dependencies.tar.xz
RUN mkdir -p /gcc-source
RUN tar -xf /gcc-with-dependencies.tar.xz -C /gcc-source --strip-components=1

RUN mkdir -p /gcc-build
WORKDIR /gcc-build

RUN <<EOT
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        host_platform=x86_64-unknown
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        host_platform=aarch64-unknown
    fi
    ../gcc-source/configure -v --build=${host_platform}-linux-gnu \
        --host=${host_platform}-linux-gnu \
        --target=${host_platform}-linux-gnu \
        --prefix=/opt/local/gcc \
        --enable-checking=release \
        --enable-languages=c,c++
EOT

RUN make
RUN make install-strip
# ensure that "cc" exists as a symlink
RUN cd /opt/local/gcc/bin/ && ln -s gcc cc
RUN tar cJf /gcc-build/gcc.tar.xz -C /opt/local/gcc --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .

ENV PATH=/opt/local/gcc/bin:$PATH

COPY /binutils-src.tar.xz /binutils-src.tar.xz
RUN mkdir -p /binutils-src
RUN tar -xf /binutils-src.tar.xz -C /binutils-src --strip-components=1
WORKDIR /binutils-src
RUN ./configure --prefix=/opt/local/binutils
RUN make
RUN make install
RUN tar cJf /gcc-build/binutils-binaries.tar.xz -C /opt/local/binutils --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .

ENV PATH=/opt/local/binutils/bin:$PATH

COPY /coreutils-src.tar.xz /coreutils-src.tar.xz
RUN mkdir -p /coreutils-src
RUN tar -xf /coreutils-src.tar.xz -C /coreutils-src --strip-components=1
WORKDIR /coreutils-src
# coreutils wants this to configure as root
ENV FORCE_UNSAFE_CONFIGURE=1
RUN ./configure ----prefix=/opt/local/coreutils
RUN make
RUN make install
RUN tar cJf /gcc-build/coreutils-binaries.tar.xz -C /opt/local/coreutils --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .