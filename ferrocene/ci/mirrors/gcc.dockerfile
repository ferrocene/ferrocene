FROM --platform=$TARGETPLATFORM centos7-base AS build
ARG TARGETPLATFORM

RUN yum -y install \
    flex bison

COPY gcc-11.tar.xz /gcc-11.tar.xz
RUN mkdir -p /opt/local/gcc-11

RUN tar -xf /gcc-11.tar.xz -C /opt/local/gcc-11
ENV PATH=/opt/local/gcc-11/bin:$PATH

COPY gcc-with-dependencies.tar.xz /gcc-with-dependencies.tar.xz
RUN mkdir -p /gcc-source
RUN tar -xf /gcc-with-dependencies.tar.xz -C /gcc-source --strip-components=1

RUN mkdir -p /gcc-build
WORKDIR /gcc-build

RUN <<EOT
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        host_platform=x86_64
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        host_platform=aarch64
    fi
    ../gcc-source/configure -v --build=${host_platform}-linux-gnu \
        --host=${host_platform}-linux-gnu \
        --target=${host_platform}-linux-gnu \
        --prefix=/opt/local/gcc \
        --enable-checking=release \
        --enable-languages=c,c++ \
        --disable-multilib 
EOT

RUN make
RUN make install-strip
RUN tar cJf /gcc-build/gcc.tar.xz -C /opt/local/gcc --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .