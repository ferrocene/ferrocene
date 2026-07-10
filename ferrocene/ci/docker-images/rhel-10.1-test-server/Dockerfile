# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# From the root of this repository, build with
# `docker build --tag rhel-10.1-test-server --file ferrocene/ci/rhel-10.1-test-server/Dockerfile .`

FROM --platform=$TARGETPLATFORM registry.access.redhat.com/ubi10:10.1

# As a multiplatform container we support all these: https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope
ARG TARGETPLATFORM
ARG BUILDPLATFORM

COPY --chmod=0700 ferrocene/ci/docker-images/ubuntu-24/run-testserver.sh /bin/run-testserver.sh

RUN mkdir -p /shared/testserver/
COPY build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/remote-test-server /shared/testserver/remote-test-server

CMD [ "sleep", "5000" ]