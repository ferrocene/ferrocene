# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# From the root of this repository, build with
# `docker build --tag rhel-10.1-test-server --file ferrocene/ci/rhel-10.1-test-server/Dockerfile .`

ARG BASE_IMAGE

FROM ${BASE_IMAGE} as base

USER root
# The following needs to be in sync with the x86-64-linux-dist-targets step

RUN mkdir -p /bin/test-server/aarch64-unknown-ferrocene.facade
COPY build/x86_64-unknown-linux-gnu/stage2-tools/aarch64-unknown-ferrocene.facade/release/remote-test-server /bin/test-server/aarch64-unknown-ferrocene.facade/remote-test-server

RUN mkdir -p /bin/test-server/aarch64r82-unknown-ferrocene.facade
COPY build/x86_64-unknown-linux-gnu/stage2-tools/aarch64r82-unknown-ferrocene.facade/release/remote-test-server /bin/test-server/aarch64r82-unknown-ferrocene.facade/remote-test-server

RUN mkdir -p /bin/test-server/aarch64v8r-unknown-ferrocene.facade
COPY build/x86_64-unknown-linux-gnu/stage2-tools/aarch64v8r-unknown-ferrocene.facade/release/remote-test-server /bin/test-server/aarch64v8r-unknown-ferrocene.facade/remote-test-server

RUN mkdir -p /bin/test-server/armv7r-ferrocene.facade-eabihf
COPY build/x86_64-unknown-linux-gnu/stage2-tools/armv7r-ferrocene.facade-eabihf/release/remote-test-server /bin/test-server/armv7r-ferrocene.facade-eabihf/remote-test-server

RUN mkdir -p /bin/test-server/thumbv7em-ferrocene.facade-eabihf
COPY build/x86_64-unknown-linux-gnu/stage2-tools/thumbv7em-ferrocene.facade-eabihf/release/remote-test-server /bin/test-server/thumbv7em-ferrocene.facade-eabihf/remote-test-server

RUN mkdir -p /bin/test-server/thumbv7em-ferrocene.facade-eabi
COPY build/x86_64-unknown-linux-gnu/stage2-tools/thumbv7em-ferrocene.facade-eabi/release/remote-test-server /bin/test-server/thumbv7em-ferrocene.facade-eabi/remote-test-server

RUN chown -R ci:ci /bin/test-server

USER ci

CMD [ "sleep", "5000" ]