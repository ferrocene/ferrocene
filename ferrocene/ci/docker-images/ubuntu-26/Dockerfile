FROM ubuntu:26.04

RUN <<-EOF
    set -ex

    CT_NG_VERS=1.28.0

    # crosstool-ng dependencies
    apt update
    apt-get install -y gcc g++ gperf bison flex texinfo help2man make libncurses5-dev \
        python3-dev autoconf automake libtool libtool-bin gawk wget bzip2 xz-utils unzip \
        patch libstdc++6 rsync git meson ninja-build

    # build crosstool-ng
    apt-get install -y curl
    mkdir /tmp/wip
    cd /tmp/wip
    curl -LO http://crosstool-ng.org/download/crosstool-ng/crosstool-ng-${CT_NG_VERS}.tar.xz
    tar xf crosstool-ng-${CT_NG_VERS}.tar.xz
    cd crosstool-ng-${CT_NG_VERS}/
    ./configure
    make -j$(nproc)
    make install
    rm -rf /tmp/wip
EOF

# bootstrap dependencies
RUN apt install -y cmake python-is-python3 ninja-build

# ct-ng cannot be used as root
RUN mkdir /x-tools && chown ubuntu:ubuntu /x-tools
USER ubuntu
RUN mkdir /tmp/wip
COPY x86_64-unknown-linux-gnu.defconfig /tmp/wip/defconfig
RUN <<-EOF
    set -ex

    CT_TARGET=x86_64-unknown-linux-gnu

    # build x-toolchain
    cd /tmp/wip
    ct-ng defconfig
    ct-ng build
EOF

# build & install zlib in x-toolchain sysroot
# LLD needs zlib support to link the compressed debuginfo found in Rust programs
RUN <<-EOF
    set -ex

    CT_TARGET=x86_64-unknown-linux-gnu
    CT_SYSROOT=/x-tools/${CT_TARGET}/${CT_TARGET}/sysroot/usr

    chmod u+w ${CT_SYSROOT}/lib
    chmod u+w ${CT_SYSROOT}/share
    chmod u+w ${CT_SYSROOT}/include

    cd /tmp/wip/.build/${CT_TARGET}/src/zlib
    ./configure --prefix=${CT_SYSROOT}
    make -j$(nproc)
    make install

    chmod u-w ${CT_SYSROOT}/lib
    chmod u-w ${CT_SYSROOT}/share
    chmod u-w ${CT_SYSROOT}/include

    rm -rf /tmp/wip
EOF
