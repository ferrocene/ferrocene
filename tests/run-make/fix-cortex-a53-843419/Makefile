# only-aarch64-unknown-none

# regression test for rust-lang/rust#118095

# run-make sets `cc` as the linker; override that default
RUSTC_LINKER=rust-lld
include ../tools.mk
RUSTC_FLAGS = --target aarch64-unknown-none -C link-arg="link.x" --print link-args

# 1. check that LLD is invoked with the errata workaround flag
# 2. check that the first two required pre-condition instructions are there
# 3. check that the third problematic instruction is NOT there. LLD will replace it with a different instruction
all:
	$(RUSTC) $(RUSTC_FLAGS) main.rs | $(CGREP) -- --fix-cortex-a53-843419
	aarch64-linux-gnu-objdump --start-address=0xff8 --stop-address=0xffc -Cd $(TMPDIR)/main | $(CGREP) -e 'adrp[[:space:]]+x0'
	aarch64-linux-gnu-objdump --start-address=0xffc --stop-address=0x1000 -Cd $(TMPDIR)/main | $(CGREP) -e 'ldr[[:space:]]+x1'
	aarch64-linux-gnu-objdump --start-address=0x1000 --stop-address=0x1004 -Cd $(TMPDIR)/main | $(CGREP) -v -e 'ldr[[:space:]]x0'
