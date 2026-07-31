# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM centos7-base AS build
ARG TARGETPLATFORM

RUN yum -y install \
    binutils \
    flex bison

ADD bootstrap-gcc.tar.xz /opt/local/bootstrap-gcc
ENV PATH=/opt/local/bootstrap-gcc/bin:$PATH

COPY gcc-with-dependencies.tar.xz /gcc-with-dependencies.tar.xz
RUN mkdir -p /gcc-source
RUN tar -xf /gcc-with-dependencies.tar.xz -C /gcc-source --strip-components=1

COPY binutils-src.tar.xz /binutils-src.tar.xz

# We need modern binutils to build an optimized compiler
# install it to the same directory as gcc so it's included
# in the same tarball
#
# We can't build binutils with gcc because it doesn't like
# pgo builds
RUN mkdir -p /binutils-src
RUN tar -xf /binutils-src.tar.xz -C /binutils-src --strip-components=1
WORKDIR /binutils-src
RUN ./configure --prefix=/opt/local/gcc
RUN make -j$(nproc)
RUN make install

ENV PATH=/opt/local/gcc/bin:$PATH

ENV LDFLAGS="-Wl,-O1 -Wl,--as-needed -Wl,--sort-common"
ENV STAGE1_CFLAGS="-mtune=native -O3 -pipe"
ENV STAGE1_CXXFLAGS="-mtune=native -O3 -pipe"
ENV BOOT_CFLAGS="-mtune=native -O3 -pipe"
ENV CFLAGS="-Wno-error=format-truncation -O3 -mtune=native -pipe"

RUN mkdir -p /gcc-build
WORKDIR /gcc-build

RUN <<EOT
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        host_platform=x86_64-unknown
        yum install -y libatomic.i686
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        host_platform=aarch64-unknown
    fi
    ../gcc-source/configure -v --build=${host_platform}-linux-gnu \
        --host=${host_platform}-linux-gnu \
        --target=${host_platform}-linux-gnu \
        --prefix=/opt/local/gcc \
        --enable-checking=release \
        --with-build-config='bootstrap-native bootstrap-lto bootstrap-O3' \
        --enable-languages=c,c++
EOT

RUN make -j$(nprocs) profiledbootstrap
RUN make install-strip
# ensure that "cc" exists as a symlink
RUN cd /opt/local/gcc/bin/ && ln -s gcc cc
RUN tar cJf /gcc-build/gcc.tar.xz -C /opt/local/gcc --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .