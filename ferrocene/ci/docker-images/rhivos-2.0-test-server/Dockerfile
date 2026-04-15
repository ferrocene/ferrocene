# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# `docker build --tag rhivos-2.0-test-server --file ferrocene/ci/docker-images/rhivos-2.0-test-server/Dockerfile .`
FROM --platform=$TARGETPLATFORM 886866542769.dkr.ecr.us-east-1.amazonaws.com/ci-docker-images:rhivos-2.0-09-04-2026

# As a multiplatform container we support all these: https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope
ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN mkdir -p /bin/test-server/aarch64-rhivos2-linux-gnu
COPY build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-rhivos2-linux-gnu/release/remote-test-server /bin/test-server/aarch64-rhivos2-linux-gnu/remote-test-server
CMD [ "sleep", "5000" ]