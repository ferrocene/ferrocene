include ../../../../../run-make-fulldeps/tools.mk

# Suffix is added before the extension
all:
	$(RUSTC) -Cextra-filename=foo --emit=mir --target $(TARGET) $(RUSTFLAGS) main.rs
	test -e $(TMPDIR)/mainfoo.mir

# ferrocene-annotations: um_rustc_C_extra_filename
