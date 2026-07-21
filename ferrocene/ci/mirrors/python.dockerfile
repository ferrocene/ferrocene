# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM amazonlinux:2

RUN yum -y install git make wget xz gcc gcc-c++ gcc10 gcc10-c++ tar patch bzip2 file openssl11-devel zlib-devel

# Select gcc10 as the compiler
ENV CC="gcc10-cc"
ENV CXX="gcc10-c++"
ENV AR="gcc10-ar"
ENV LD="gcc10-ld"

# We need a newer Python than what's included in Ubuntu 20.04, so we have to
# build it from source. Python 3.12.x has been chosen as it's the same minor
# release shipped with Ubuntu 24.04 LTS.

RUN mkdir /python
WORKDIR /python

COPY python-source.tar.xz /python-source.tar.xz

RUN tar  -C /python -xf /python-source.tar.xz --strip-components=1

RUN ./configure --enable-optimizations --prefix=/python-install

RUN make -j$(nproc)
RUN make install
RUN echo "Creating $(du -hs .) archive"
RUN tar -C /python-install -cJf python-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .