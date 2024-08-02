# only-aarch64
# ignore-apple On Mac, we can be sure the CPU is never an a53 (Apple does not make those)

# regression test for rust-lang/rust#118095

# run-make sets `cc` as the linker using this variable;
# clear it to let the target use its default linker
RUSTC_LINKER=

include ../tools.mk
RUSTC_FLAGS = --target $(TARGET) --print link-args -C panic=abort

# this target directly uses LLD as the linker; the others use GCC
# the GCC targets need -nostartfiles because our program is basically a startup object
ifeq ($(TARGET),aarch64-unknown-none)
  RUSTC_FLAGS += -C link-arg="link.x"
else
  RUSTC_FLAGS += -C link-arg="-Wl,link.x" -C link-arg="-nostartfiles"
endif

# 1. check that LLD is invoked with the errata workaround flag
# 2. check that the first two required pre-condition instructions are there
# 3. check that the third problematic instruction is NOT there. LLD will replace it with a different instruction
all:
	$(RUSTC) $(RUSTC_FLAGS) main.rs | $(CGREP) -- --fix-cortex-a53-843419
	aarch64-linux-gnu-objdump --start-address=0xff8 --stop-address=0xffc -Cd $(TMPDIR)/main | $(CGREP) -e 'adrp[[:space:]]+x0'
	aarch64-linux-gnu-objdump --start-address=0xffc --stop-address=0x1000 -Cd $(TMPDIR)/main | $(CGREP) -e 'ldr[[:space:]]+x1'
	aarch64-linux-gnu-objdump --start-address=0x1000 --stop-address=0x1004 -Cd $(TMPDIR)/main | $(CGREP) -v -e 'ldr[[:space:]]x0'
