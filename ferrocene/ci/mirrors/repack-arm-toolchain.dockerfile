# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM --platform=$TARGETPLATFORM centos7-base AS build
ARG TARGETPLATFORM

COPY /arm-gnu-toolchain.tar.xz /arm-gnu-toolchain.tar.xz
RUN mkdir -p /arm-toolchain
RUN tar -xf /arm-gnu-toolchain.tar.xz -C /arm-toolchain --strip-components=1

RUN mkdir -p /repacked
RUN tar cJf /repacked/arm-gnu-toolchain-repacked.tar.xz -C /arm-toolchain --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .