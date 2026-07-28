# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM centos7-base AS build
ARG TARGETPLATFORM

RUN yum -y install \
    flex bison \
    gcc gcc-c++
    
COPY gcc-11-with-dependencies.tar.xz /gcc-11-with-dependencies.tar.xz
RUN mkdir -p /gcc-source
RUN tar -xf /gcc-11-with-dependencies.tar.xz -C /gcc-source --strip-components=1

RUN mkdir -p /gcc-build
WORKDIR /gcc-build

RUN <<EOT
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        host_platform=x86_64
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        host_platform=aarch64
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