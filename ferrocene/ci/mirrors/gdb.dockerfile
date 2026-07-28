# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

FROM centos7-python-builder AS base

ARG PYTHON_VERSION

RUN yum -y install \
        gmp-devel mpfr-devel

COPY gdb-source.tar.xz /gdb-source.tar.xz
RUN mkdir -p /gdb
RUN tar -xf /gdb-source.tar.xz -C /gdb --strip-components=1
WORKDIR /gdb

# you can remove all of the following if building a recent gdb

# readline in gdb12 is incompatible with the C21 standard
ENV CFLAGS="-g -O2 -std=gnu17"
# gdb 12 is incompatible with newer C++ standards
ENV CXXFLAGS="-g -O2 -std=gnu++17"

# gdb 12 needs this patch to work with a recent gcc
# see https://sourceware.org/pipermail/gdb-cvs/2022-August/053725.html
RUN <<EOT 
        echo '
diff --git a/sim/aarch64/cpustate.h b/sim/aarch64/cpustate.h
index 1d25b3af15a..94e0bc80333 100644
--- a/sim/aarch64/cpustate.h
+++ b/sim/aarch64/cpustate.h
@@ -307,7 +307,7 @@ extern void        aarch64_save_LR (sim_cpu *);
 /* Flag register accessors.  */
 extern uint32_t    aarch64_get_CPSR       (sim_cpu *);
 extern void        aarch64_set_CPSR       (sim_cpu *, uint32_t);
-extern uint32_t    aarch64_get_CPSR_bits  (sim_cpu *, uint32_t);
+extern uint32_t    aarch64_get_CPSR_bits  (sim_cpu *, FlagMask);
 extern void        aarch64_set_CPSR_bits  (sim_cpu *, uint32_t, uint32_t);
 extern uint32_t    aarch64_test_CPSR_bit  (sim_cpu *, FlagMask);
 extern void        aarch64_set_CPSR_bit   (sim_cpu *, FlagMask);' | patch -p1
EOT

RUN ./configure --with-python=/opt/python/${PYTHON_VERSION}/bin/python3 --prefix=/gdb-install
RUN make

RUN make install
RUN echo "Creating $(du -hs .) archive"
RUN tar -C /gdb-install -cJf gdb-binaries.tar.xz --checkpoint=10000 --checkpoint-action=echo="#%u: %T" .
