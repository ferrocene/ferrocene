# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM centos7-base AS build
ARG TARGETPLATFORM

ADD gcc.tar.xz /opt/local/gcc
RUN tar -xf /gcc.tar.xz -C /opt/local/gcc
ADD binutils.tar.xz /opt/local/
ADD openssl.tar.xz /usr/local

ENV PATH=/opt/local/gcc/bin:/opt/local/bin:$PATH
ENV PKG_CONFIG_PATH=/usr/local/lib64/pkgconfig/

# add /usr/local/lib to ld's path
RUN echo '/usr/local/lib/' > /etc/ld.so.conf.d/openssl.conf
RUN echo '/usr/local/lib64' >> /etc/ld.so.conf.d/openssl.conf
RUN ldconfig

RUN <<-EOF
    set -eu

    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        host_platform=x86_64
        curl -Lo ninja-build.zip https://github.com/ninja-build/ninja/releases/download/v1.12.1/ninja-linux.zip
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        host_platform=aarch64
        curl -Lo ninja-build.zip https://github.com/ninja-build/ninja/releases/download/v1.12.1/ninja-linux-aarch64.zip
    fi
    unzip ninja-build.zip
    mv ninja /usr/local/bin
    rm -f ninja-build.zip
EOF