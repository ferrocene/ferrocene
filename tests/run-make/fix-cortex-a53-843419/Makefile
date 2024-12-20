# only-aarch64
# ignore-apple On Mac, we can be sure the CPU is never an a53 (Apple does not make those)

# regression test for rust-lang/rust#118095

# run-make sets `cc` as the linker using this variable;
# clear it to let the target use its default linker
RUSTC_LINKER=

include ../tools.mk
RUSTC_FLAGS = --target $(TARGET) --print link-args -C panic=abort

# this target directly uses LLD as the linker; the others use GCC
# the GCC targets need -nostartfiles (or equivalent) because our program is basically a startup object
ifeq ($(TARGET),aarch64-unknown-none)
  RUSTC_FLAGS += -C link-arg="link.x"
else
ifeq ($(TARGET),aarch64-unknown-nto-qnx710)
  RUSTC_FLAGS += -C link-arg="-Wl,-Tlink.x" -C link-arg="-nostartup"
else
  RUSTC_FLAGS += -C link-arg="-Wl,link.x" -C link-arg="-nostartfiles"
endif
endif

all:
  # 1. check that linker is invoked with the errata workaround flag
	$(RUSTC) $(RUSTC_FLAGS) main.rs | $(CGREP) -- --fix-cortex-a53-843419
  # # Targets that use GCC / LD as linker
ifeq ($(TARGET),no-target-uses-gnu-ld)
  # 2. check that the first two required pre-condition instructions (ADRP then LDR) are NOT there. GCC will replace ADRP with ADR
	aarch64-linux-gnu-objdump --start-address=0xff8 --stop-address=0xffc -Cd $(TMPDIR)/main | $(CGREP) -v -e 'adrp[[:space:]]+x0'
	aarch64-linux-gnu-objdump --start-address=0xff8 --stop-address=0xffc -Cd $(TMPDIR)/main | $(CGREP) -e 'adr[[:space:]]+x0'
	aarch64-linux-gnu-objdump --start-address=0xffc --stop-address=0x1000 -Cd $(TMPDIR)/main | $(CGREP) -e 'ldr[[:space:]]+x1'
else
  # # Targets that use LLD as linker
  # 2. check that the first two required pre-condition instructions (ADRP then LDR) are there
  # 3. check that the third problematic instruction (LDR) is NOT there. LLD will replace it with a different instruction
	aarch64-linux-gnu-objdump --start-address=0xff8 --stop-address=0xffc -Cd $(TMPDIR)/main | $(CGREP) -e 'adrp[[:space:]]+x0'
	aarch64-linux-gnu-objdump --start-address=0xffc --stop-address=0x1000 -Cd $(TMPDIR)/main | $(CGREP) -e 'ldr[[:space:]]+x1'
	aarch64-linux-gnu-objdump --start-address=0x1000 --stop-address=0x1004 -Cd $(TMPDIR)/main | $(CGREP) -v -e 'ldr[[:space:]]x0'
endif

# Example assembly
#
# - BAD: ADRP @ 0xff8, LDR, LDR
#
# ```
# 0000000000000ff0 <_start>:
#     ff0:	d503201f 	nop
#     ff4:	d503201f 	nop
#     ff8:	b0000000 	adrp	x0, 1000 <_start+0x10>
#     ffc:	f9400021 	ldr	x1, [x1]
#    1000:	f940b800 	ldr	x0, [x0, #368]
#    1004:	d65f03c0 	ret
# ```
#
# - LLD fix: replace LDR with a B(ranch) instruction
#
# ```
# 0000000000000ff0 <_start>:
#     ff0:	d503201f 	nop
#     ff4:	d503201f 	nop
#     ff8:	b0000000 	adrp	x0, 1000 <_start+0x10>
#     ffc:	f9400021 	ldr	x1, [x1]
#    1000:	14000002 	b	1008 <__CortexA53843419_1000>
#    1004:	d65f03c0 	ret
#
# 0000000000001008 <__CortexA53843419_1000>:
#    1008:	f9400800 	ldr	x0, [x0, #16]
#    100c:	17fffffe 	b	1004 <_start+0x14>
#
# ```
#
# - GNU LD fix: replace ADRP with ADR

# ```
# 0000000000000ff0 <_start>:
#     ff0:	d503201f 	nop
#     ff4:	d503201f 	nop
#     ff8:	10008040 	adr	x0, 2000 <e843419@0002_00000011_10+0xff0>
#     ffc:	f9400021 	ldr	x1, [x1]
#    1000:	f940b800 	ldr	x0, [x0, #368]
#    1004:	d65f03c0 	ret
#    1008:	14000400 	b	2008 <e843419@0002_00000011_10+0xff8>
#    100c:	d503201f 	nop
#
# ```
