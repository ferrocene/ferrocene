FROM --platform=$TARGETPLATFORM centos7-python-builder AS build
ARG TARGETPLATFORM

RUN yum -y install \
    flex bison \
    texinfo help2man \
    libtool \
    libatomic.i686 \
    ncurses-devel

#RUN yum install -y autoconf gperf bison flex texinfo help2man libtool \
#    ncurses-devel python36-devel perl-Thread-Queue git rsync

COPY /crosstool-ng.tar.xz /crosstool-ng.tar.xz
RUN mkdir -p /crosstool-ng
RUN tar -xf /crosstool-ng.tar.xz -C /crosstool-ng --strip-components=1
WORKDIR /crosstool-ng
RUN ./configure --prefix=/opt/crosstool-ng
RUN make
RUN make install
ENV PATH=$PATH:/opt/crosstool-ng/bin

ADD /ct-ng-sources.tar.xz /home/ci/src

USER ci
WORKDIR /home/ci

FROM build AS aarch64-linux-gnu

COPY /aarch64-linux-gnu.defconfig defconfig
RUN ct-ng defconfig
RUN ct-ng build
RUN tar cJf /home/ci/ct-ng-aarch64-linux-gnu.tar.xz -C /home/ci/x-tools/aarch64-linux-gnu --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .

FROM build AS s390x-linux-gnu

COPY /s390x-linux-gnu.defconfig defconfig
RUN ct-ng defconfig
RUN ct-ng build
RUN tar cJf /home/ci/ct-ng-s390x-linux-gnu.tar.xz -C /home/ci/x-tools/s390x-linux-gnu --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .

FROM build AS powerpc64le-linux-gnu

COPY /powerpc64le-linux-gnu.defconfig defconfig
RUN ct-ng defconfig
RUN ct-ng build
RUN tar cJf /home/ci/ct-ng-powerpc64le-linux-gnu.tar.xz -C /home/ci/x-tools/powerpc64le-linux-gnu --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .

FROM build AS risc64-unknown-linux-gnu

COPY /risc64-unknown-linux-gnu.defconfig defconfig
RUN ct-ng defconfig
RUN ct-ng build
RUN tar cJf /home/ci/ct-ng-risc64-unknown-linux-gnu.tar.xz -C /home/ci/x-tools/risc64-unknown-linux-gnu --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .