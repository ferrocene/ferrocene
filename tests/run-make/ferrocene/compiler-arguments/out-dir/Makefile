include ../../../../run-make-fulldeps/tools.mk

all:
	mkdir $(TMPDIR)/output_dir
	$(BARE_RUSTC) main.rs --crate-type bin --out-dir=$(TMPDIR)/output_dir --target $(TARGET) $(RUSTFLAGS)
	test -d $(TMPDIR)/output_dir
	test -f $(TMPDIR)/output_dir/main

# ferrocene-annotations: um_rustc_out_dir
