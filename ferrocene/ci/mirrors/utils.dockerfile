# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM centos7-python-builder AS build
ARG TARGETPLATFORM

RUN mkdir -p /build

COPY /zstd-src.tar.gz /zstd-src.tar.gz
RUN mkdir -p /zstd-src
RUN tar -xf /zstd-src.tar.gz -C /zstd-src --strip-components=1
WORKDIR /zstd-src/build/meson
RUN meson setup -Dbin_programs=true -Dbin_contrib=true builddir
WORKDIR /zstd-src/build/meson/builddir
ENV DESTDIR=/zstd-build
RUN ninja
RUN ninja install
RUN tar cJf /build/zstd-binaries.tar.xz -C /zstd-build/usr/local/ --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .

COPY /coreutils-src.tar.xz /coreutils-src.tar.xz
RUN mkdir -p /coreutils-src
RUN tar -xf /coreutils-src.tar.xz -C /coreutils-src --strip-components=1
WORKDIR /coreutils-src
# coreutils wants this to configure as root
ENV FORCE_UNSAFE_CONFIGURE=1
ENV DESTDIR=/
RUN ./configure --prefix=/opt/local/coreutils
RUN make -j$(nproc)
RUN make install
RUN tar cJf /build/coreutils-binaries.tar.xz -C /opt/local/coreutils --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .