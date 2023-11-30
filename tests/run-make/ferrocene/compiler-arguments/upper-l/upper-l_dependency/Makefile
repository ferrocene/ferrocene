include ../../../../../run-make-fulldeps/tools.mk

all:
	mkdir $(TMPDIR)/L_dependency_dir
	$(BARE_RUSTC) operations.rs --crate-type lib --out-dir $(TMPDIR)/L_dependency_dir --target $(TARGET) $(RUSTFLAGS)
	$(BARE_RUSTC) factorial.rs --crate-type lib -L crate=$(TMPDIR)/L_dependency_dir --out-dir $(TMPDIR)/L_fact_dir --target $(TARGET) $(RUSTFLAGS)
	$(RUSTC) main.rs --crate-type bin -L crate=$(TMPDIR)/L_fact_dir -L dependency=$(TMPDIR)/L_dependency_dir --target $(TARGET) $(RUSTFLAGS)
	$(call RUN,main)

# ferrocene-annotations: um_rustc_L
