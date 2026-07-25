FROM --platform=$TARGETPLATFORM centos7-base AS build

COPY gcc.tar.xz /gcc.tar.xz
RUN mkdir -p /opt/local/gcc
RUN tar -xf /gcc.tar.xz -C /opt/local/gcc --strip-components=3
ENV PATH=/opt/local/gcc/bin:$PATH