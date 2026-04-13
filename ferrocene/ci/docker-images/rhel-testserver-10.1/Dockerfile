# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# From the root of this repository, build with
# `docker build --tag ubuntu-24 --file ferrocene/ci/docker-images/ubuntu-24/Dockerfile .`

FROM --platform=$TARGETPLATFORM registry.access.redhat.com/ubi10:10.1

# As a multiplatform container we support all these: https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope
ARG TARGETPLATFORM
ARG BUILDPLATFORM

COPY build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-unknown-linux-gnu/release/remote-test-server /bin/remote-testserver
CMD [ "/bin/remote-testserver" ]