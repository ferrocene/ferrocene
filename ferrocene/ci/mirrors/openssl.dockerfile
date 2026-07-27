FROM --platform=$TARGETPLATFORM centos7-base AS build

RUN yum -y install perl-core

COPY gcc.tar.xz /gcc.tar.xz
RUN mkdir -p /opt/local/gcc
RUN tar -xf /gcc.tar.xz -C /opt/local/gcc
ENV PATH=/opt/local/gcc/bin:$PATH


COPY openssl.tar.gz /openssl.tar.gz
RUN mkdir -p /openssl-build
RUN tar -xf /openssl.tar.gz -C /openssl-build --strip-components=1
WORKDIR /openssl-build

RUN ./Configure --prefix=/usr/local --openssldir=/usr/local/ssl  '-Wl,-rpath,$(LIBRPATH)'
RUN make
RUN make install
# We know that /usr/local only contains our freshly built openssl
RUN tar -C /usr/local/ -cJf openssl-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .