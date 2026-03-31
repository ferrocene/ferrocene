# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM ubuntu:20.04

ENV VERSION=3.21.1
ENV PACKAGE_URL="https://github.com/Kitware/CMake/releases/download/v${VERSION}/cmake-${VERSION}.tar.gz"
ENV SHA="fac3915171d4dff25913975d712f76e69aef44bf738ba7b976793a458b4cfed4"

RUN apt update && apt install -y curl
RUN curl -LO $PACKAGE_URL

RUN echo "$SHA cmake-${VERSION}.tar.gz" | sha256sum -c
RUN mkdir cmake
WORKDIR cmake
RUN tar --strip-components=1 -xzf ../cmake-${VERSION}.tar.gz

RUN mkdir /cmake-install
RUN apt update && apt install -y clang make libssl-dev
RUN ./configure --prefix=/cmake-install
RUN make -j$(nproc)
RUN make install
RUN tar -C /cmake-install -cJf cmake-${VERSION}.tar.xz .
