# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM ubuntu:20.04

# We need a newer Python than what's included in Ubuntu 20.04, so we have to
# build it from source. Python 3.12.x has been chosen as it's the same minor
# release shipped with Ubuntu 24.04 LTS.
ENV PACKAGE_URL="https://www.python.org/ftp/python/3.12.3/Python-3.12.3.tar.xz"
ENV SHA="56bfef1fdfc1221ce6720e43a661e3eb41785dd914ce99698d8c7896af4bdaa1"
ENV DEBIAN_FRONTEND=noninteractive

RUN mkdir python
WORKDIR python

RUN apt update && apt install -y curl
RUN curl -OL $PACKAGE_URL && echo "$SHA Python-3.12.3.tar.xz" | sha256sum -c

RUN apt update && apt install -y xz-utils
RUN tar xf Python-3.12.3.tar.xz --strip-components=1

RUN apt update && apt install -y gcc pkg-config zlib1g-dev libssl-dev
RUN ./configure --enable-optimizations --prefix=/python-install

RUN apt update && apt install -y make
RUN make -j$(nproc)
RUN make install
RUN echo "Creating $(du -hs .) archive"
RUN tar -C /python-install -cJf python-3.12.3.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .
