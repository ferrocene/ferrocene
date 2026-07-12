# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# `docker build --tag rhivos-2.0-test-server --file ferrocene/ci/docker-images/rhivos-2.0-test-server/Dockerfile .`
FROM --platform=$TARGETPLATFORM 886866542769.dkr.ecr.us-east-1.amazonaws.com/ci-docker-images:rhivos-2.0-19-04-2026

# As a multiplatform container we support all these: https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope
ARG TARGETPLATFORM
ARG BUILDPLATFORM

COPY --chmod=0700 ferrocene/ci/docker-images/ubuntu-24/run-testserver.sh /bin/run-testserver.sh

CMD [ "sleep", "5000" ]