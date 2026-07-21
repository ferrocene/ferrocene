# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM ubuntu:20.04

# We need a newer Python than what's included in Ubuntu 20.04, so we have to
# build it from source. Python 3.12.x has been chosen as it's the same minor
# release shipped with Ubuntu 24.04 LTS.

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y xz-utils gcc pkg-config zlib1g-dev libssl-dev

RUN mkdir /python
WORKDIR /python

COPY python-source.tar.xz /python-source.tar.xz

RUN tar  -C /python -xf /python-source.tar.xz --strip-components=1

RUN ./configure --enable-optimizations --prefix=/python-install

RUN apt update && apt install -y make
RUN make -j$(nproc)
RUN make install
RUN echo "Creating $(du -hs .) archive"
RUN tar -C /python-install -cJf python-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .