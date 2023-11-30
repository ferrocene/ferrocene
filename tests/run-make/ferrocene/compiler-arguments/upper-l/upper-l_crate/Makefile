include ../../../../../run-make-fulldeps/tools.mk

all:
	mkdir $(TMPDIR)/L_crate_dir
	$(BARE_RUSTC) operations.rs --crate-type lib --out-dir $(TMPDIR)/L_crate_dir --target $(TARGET) $(RUSTFLAGS)
	$(RUSTC) main.rs --crate-type bin -L crate=$(TMPDIR)/L_crate_dir --target $(TARGET) $(RUSTFLAGS)
	$(call RUN,main)

# ferrocene-annotations: um_rustc_L
