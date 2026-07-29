# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM centos7-base AS build
ARG TARGETPLATFORM

ADD gcc.tar.xz /opt/local/gcc
ENV PATH=/opt/local/gcc/bin:$PATH

RUN mkdir -p /build

COPY /binutils-src.tar.xz /binutils-src.tar.xz
RUN mkdir -p /binutils-src
RUN tar -xf /binutils-src.tar.xz -C /binutils-src --strip-components=1
WORKDIR /binutils-src
RUN ./configure --prefix=/opt/local/binutils
RUN make -j$(nproc)
RUN make install
RUN tar cJf /build/binutils-binaries.tar.xz -C /opt/local/binutils --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .

ENV PATH=/opt/local/binutils/bin:$PATH

COPY /coreutils-src.tar.xz /coreutils-src.tar.xz
RUN mkdir -p /coreutils-src
RUN tar -xf /coreutils-src.tar.xz -C /coreutils-src --strip-components=1
WORKDIR /coreutils-src
# coreutils wants this to configure as root
ENV FORCE_UNSAFE_CONFIGURE=1
RUN ./configure --prefix=/opt/local/coreutils
RUN make -j$(nproc)
RUN make install
RUN tar cJf /build/coreutils-binaries.tar.xz -C /opt/local/coreutils --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .