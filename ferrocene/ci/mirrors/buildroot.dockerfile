# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM centos7-python-builder AS build

ARG TARGETPLATFORM

RUN mkdir -p /usr/local/

# We need zstd to enable the lto zstd features
ADD /zstd.tar.xz /usr/local/

COPY /cmake.tar.gz /cmake.tar.gz
RUN tar -xf cmake.tar.gz -C /usr/local/ --strip-components=1

RUN mkdir -p /llvm-src
COPY /llvm-project.src.tar.xz /llvm-project.src.tar.xz
RUN tar -xf  /llvm-project.src.tar.xz -C /llvm-src --strip-components=1

RUN mkdir -p /llvm-build
WORKDIR /llvm-build

ENV GCC_VERSION=16.1.0

# For whatever reason the default set of include paths for clang is different
# than that of gcc. As a result we need to manually include our sysroot's
# include path, /ferrocene-buildroot/include, to clang's default include path.
ENV INC="/ferrocene-buildroot/include:/usr/include/x86_64-linux-gnu/:/usr/local/include:/usr/include"

ENV LLVM_BUILD_TARGETS="X86;WebAssembly;ARM;AArch64"

RUN <<EOT
    set -xe

    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        export GCC_BUILD_TARGET=x86_64-pc-linux-gnu
        export GCC_PLUGIN_TARGET=x86_64-pc-linux-gnu
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        export GCC_BUILD_TARGET=aarch64-unknown-linux-gnu
        export GCC_PLUGIN_TARGET=aarch64-unknown-linux-gnu
    fi
    
    cmake -S ../llvm-src/llvm -B /llvm-build \
      -DCMAKE_C_COMPILER=/ferrocene-buildroot/bin/gcc\
      -DCMAKE_CXX_COMPILER=/ferrocene-buildroot/bin/g++ \
      -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_INSTALL_PREFIX=/ferrocene-buildroot \
      -DCOMPILER_RT_BUILD_SANITIZERS=OFF \
      -DCOMPILER_RT_BUILD_XRAY=OFF \
      -DCOMPILER_RT_BUILD_MEMPROF=OFF \
      -DCOMPILER_RT_BUILD_CTX_PROFILE=OFF \
      -DLLVM_TARGETS_TO_BUILD=$LLVM_BUILD_TARGETS \
      -DLLVM_INCLUDE_BENCHMARKS=OFF \
      -DLLVM_INCLUDE_TESTS=OFF \
      -DLLVM_INCLUDE_EXAMPLES=OFF \
      -DLLVM_ENABLE_PROJECTS="clang;lld;bolt" \
      -DLLVM_ENABLE_RUNTIMES="compiler-rt" \
      -DLLVM_BINUTILS_INCDIR="/ferrocene-buildroot/lib/gcc/$GCC_PLUGIN_TARGET/$GCC_VERSION/plugin/include/" \
      -DRUNTIMES_CMAKE_ARGS="-DCMAKE_CXX_FLAGS=\"--gcc-toolchain=/ferrocene-buildroot\"" \
      -DC_INCLUDE_DIRS="$INC"
EOT

RUN make -j$(nproc)

RUN make install

RUN <<EOT
    set -xe

    if [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        # Our configuration expects the compiler to be named
        # aarch64-linux-gnu-gcc
        cd /ferrocene-buildroot/bin
        ln -s aarch64-unknown-linux-gnu-gcc aarch64-linux-gnu-gcc && true
        ln -s aarch64-unknown-linux-gnu-c++ aarch64-linux-gnu-c++ && true
    fi

EOT

# The ferrocene buildroot contains clang and gcc
RUN tar cJf /ferrocene-buildroot.tar.xz -C /ferrocene-buildroot --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .