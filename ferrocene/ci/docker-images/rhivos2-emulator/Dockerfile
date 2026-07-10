# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# `docker build --tag rhivos2-emulator --file ferrocene/ci/docker-images/rhivos2-emulator/Dockerfile .`
FROM --platform=$TARGETPLATFORM 886866542769.dkr.ecr.us-east-1.amazonaws.com/ci-docker-images:rhivos-2.0-19-04-2026

# As a multiplatform container we support all these: https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope
ARG TARGETPLATFORM
ARG BUILDPLATFORM

COPY --chmod=0700 ferrocene/ci/docker-images/emulator/run-testserver.sh /bin/run-testserver.sh

RUN mkdir -p /shared/testserver/
COPY build/aarch64-unknown-linux-gnu/stage2-tools/aarch64-rhivos2-linux-gnu/release/remote-test-server /shared/testserver/remote-test-server
CMD [ "sleep", "5000" ]
