# Emits two asm files, one with no-vectorize-loops enabled and one without.
# Optimization is on for both, just no-vectorize-loops is different.
# The resulting asm must be different.

include ../../../../../run-make-fulldeps/tools.mk

all:
	mkdir -p $(TMPDIR)/out_dir_with
	$(BARE_RUSTC) example.rs --crate-type lib -C opt-level=3 -C no-vectorize-loops \
		--out-dir=$(TMPDIR)/out_dir_with \
		--emit asm --target $(TARGET) $(RUSTFLAGS)
	mkdir -p $(TMPDIR)/out_dir_without
	$(BARE_RUSTC) example.rs --crate-type lib -C opt-level=3 \
		--out-dir=$(TMPDIR)/out_dir_without \
		--emit asm --target $(TARGET) $(RUSTFLAGS)
	diff $(TMPDIR)/out_dir_with/example.s $(TMPDIR)/out_dir_without/example.s || [ ! $$? -eq 0 ]

# ferrocene-annotations: um_rustc_C_no_vectorize_loops
