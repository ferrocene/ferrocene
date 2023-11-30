# needs-dynamic-linking

include ../../../../../run-make-fulldeps/tools.mk

# Check default behavior is rpath=no
all:
	$(RUSTC) main.rs             -Cextra-filename=-default --crate-type bin --target $(TARGET) $(RUSTFLAGS)
	$(RUSTC) main.rs -Crpath=yes -Cextra-filename=-yes     --crate-type bin --target $(TARGET) $(RUSTFLAGS)
	$(RUSTC) main.rs -Crpath=no  -Cextra-filename=-no      --crate-type bin --target $(TARGET) $(RUSTFLAGS)
	$(RUSTC) main.rs -Crpath     -Cextra-filename=-empty   --crate-type bin --target $(TARGET) $(RUSTFLAGS)
	# Default is no
	$(DIFF) $(TMPDIR)/main-default $(TMPDIR)/main-no
	# Empty is an alias of yes
	$(DIFF) $(TMPDIR)/main-empty $(TMPDIR)/main-yes
	# Binaries are different when rpath is different
	if $(DIFF) $(TMPDIR)/main-yes $(TMPDIR)/main-no; then exit 1; fi

# ferrocene-annotations: um_rustc_C_rpath
