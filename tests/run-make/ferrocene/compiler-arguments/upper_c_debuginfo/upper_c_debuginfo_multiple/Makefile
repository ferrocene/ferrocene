include ../../../../../run-make-fulldeps/tools.mk

# Multiple debuginfo arguments are passed with different options.
# The last one overrides the previous.

all:
	# Compile with a single debuginfo flag
	$(RUSTC) --target $(TARGET) $(RUSTFLAGS) -Cdebuginfo=0 -o $(TMPDIR)/0 main.rs
	$(RUSTC) --target $(TARGET) $(RUSTFLAGS) -Cdebuginfo=2 -o $(TMPDIR)/2 main.rs
	# Compile with multiple debuginfo flags
	$(RUSTC) --target $(TARGET) $(RUSTFLAGS) -Cdebuginfo=0 -Cdebuginfo=2 -o $(TMPDIR)/02 main.rs
	$(RUSTC) --target $(TARGET) $(RUSTFLAGS) -Cdebuginfo=2 -Cdebuginfo=0 -o $(TMPDIR)/20 main.rs
	# Compare build artifacts, they must be equal
	diff $(TMPDIR)/2 $(TMPDIR)/02
	diff $(TMPDIR)/0 $(TMPDIR)/20
	# Compare build artifacts, they must be different
	diff $(TMPDIR)/02 $(TMPDIR)/20; test ! $$? -eq 0

# ferrocene-annotations: um_rustc_C_debuginfo
