FROM --platform=$TARGETPLATFORM centos7-python-builder AS build
ARG TARGETPLATFORM

RUN yum -y install \
    flex bison \
    texinfo help2man \
    ncurses-devel

#RUN yum install -y autoconf gperf bison flex texinfo help2man libtool \
#    ncurses-devel python36-devel perl-Thread-Queue git rsync

ADD /crosstool-ng.tar.xz /crosstool-ng
WORKDIR /crosstool-ng
RUN ./configure --prefix=/opt/crosstool-ng
RUN make
RUN make install
ENV PATH=$PATH:/opt/crosstool-ng/bin

USER ci
WORKDIR /home/ci

FROM build AS aarch64-linux-gnu

ADD /ctn-ng-aarch64-linux-gnu-sources.tar.xz /home/ci/src
COPY /aarch64-linux-gnu.defconfig defconfig
RUN ct-ng defconfig
RUN ct-ng build
RUN tar cJf /home/ci/ctn-ng-aarch64-linux-gnu.tar.xz -C /home/ci/x-tools/aarch64-linux-gnu --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .