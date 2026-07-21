# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM amazonlinux:2

RUN yum -y install git make wget xz gcc gcc-c++ gcc10 gcc10-c++ tar patch bzip2 file openssl11-devel zlib-devel 

# Select gcc10 as the compiler
ENV CC="gcc10-cc"
ENV CXX="gcc10-c++"
ENV AR="gcc10-ar"
ENV LD="gcc10-ld"

COPY cmake-source.tar.gz /cmake-source.tar.gz
RUN mkdir /cmake
RUN mkdir /cmake-install
WORKDIR /cmake

RUN tar --strip-components=1 -C /cmake -xzf /cmake-source.tar.gz

RUN ./configure --prefix=/cmake-install
RUN make -j$(nproc)
RUN make install
RUN tar -C /cmake-install -cJf cmake-binaries.tar.xz .