include ../../../../../run-make-fulldeps/tools.mk

all:
	mkdir $(TMPDIR)/L_all_dir
	$(BARE_RUSTC) operations.rs --crate-type lib --out-dir $(TMPDIR)/L_all_dir --target $(TARGET) $(RUSTFLAGS)
	$(RUSTC) main.rs --crate-type bin -L all=$(TMPDIR)/L_all_dir --target $(TARGET) $(RUSTFLAGS)
	$(call RUN,main)

# ferrocene-annotations: um_rustc_L
