include ../../../../../run-make-fulldeps/tools.mk

all:
	$(RUSTC) -Cextra-filename=foo --target $(TARGET) $(RUSTFLAGS) main.rs
	test -e $(TMPDIR)/mainfoo

# ferrocene-annotations: um_rustc_C_extra_filename
