# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This dockerfile expects a musl-cross source tree in the work directory, optionally
# with the sources tarballs in the source/ subdirectory
FROM ubuntu:20.04 AS base
ARG TARGETPLATFORM
ARG TARGETVERSION=v0.9.11

RUN apt update && apt install -y git \
  make \
  wget \
  xz-utils \
  gcc-aarch64-linux-gnu \
  g++-aarch64-linux-gnu \
  gcc-x86-64-linux-gnu \
  g++-x86-64-linux-gnu
ADD musl-cross-make/ /musl-cross-make/
WORKDIR /musl-cross-make
RUN git checkout fd6be58297ee21fcba89216ccd0d4aca1e3f1c5c # v0.9.11

ENV LINUX_HEADERS_SITE="https://mirrors.2f30.org/sabotage/tarballs/"
ENV GNU_SITE="https://gcc.gnu.org/"
ENV MUSL_VER=1.2.5
RUN make extract_all

# we need to build the code as relocatable / position independent (-fPIC) or else we get
# linker errors with the x86_64 MUSL target. the `-g1` flag lets backtraces cross into C libraries
ENV CFLAGS="-fPIC -g1"

FROM base AS aarch64
ARG TARGETPLATFORM
ARG TARGETVERSION=v0.9.11

ENV TARGET=aarch64-linux-musl
RUN make

RUN make install
RUN echo "Creating $(du -hs output) archive"
RUN tar -C output -cJf musl-cross-make-aarch64-${TARGETVERSION}.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .

FROM base AS x86_64
ARG TARGETPLATFORM
ARG TARGETVERSION=v0.9.11

ENV TARGET=x86_64-linux-musl
RUN make

RUN make install
RUN echo "Creating $(du -hs output) archive"

RUN tar -C output -cJf musl-cross-make-x86_64-${TARGETVERSION}.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .

# This needs to be an executable image since we want to start it, so we can copy the tarballs out
FROM ubuntu:20.04 AS tarballs
ARG TARGETPLATFORM
ARG TARGETVERSION=v0.9.11

COPY --from=aarch64 /musl-cross-make/musl-cross-make-aarch64-${TARGETVERSION}.tar.xz .
COPY --from=x86_64 /musl-cross-make/musl-cross-make-x86_64-${TARGETVERSION}.tar.xz .