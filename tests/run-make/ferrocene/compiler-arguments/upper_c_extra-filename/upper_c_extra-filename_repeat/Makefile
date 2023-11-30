# Every occurrence of -Cextra-filename overrides the previous one.
# Testing only in linux, other OS may add a suffix
# only-linux
include ../../../../../run-make-fulldeps/tools.mk

all:
	$(RUSTC) --target $(TARGET) -Cextra-filename=foo -Cextra-filename=bar main.rs
	test ! -e $(TMPDIR)/mainfoo
	test -e $(TMPDIR)/mainbar

# ferrocene-annotations: um_rustc_C_extra_filename
