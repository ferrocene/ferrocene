include ../../../../../run-make-fulldeps/tools.mk

# Check linker arguments
all:
	$(RUSTC) main.rs --crate-type bin \
		--target $(TARGET) $(RUSTFLAGS) --print link-args | $(CGREP) -v -rpath
	$(RUSTC) main.rs --crate-type bin -Crpath=yes \
		--target $(TARGET) $(RUSTFLAGS) --print link-args | $(CGREP) -rpath
	$(RUSTC) main.rs --crate-type bin -Crpath=no \
		--target $(TARGET) $(RUSTFLAGS) --print link-args | $(CGREP) -v -rpath
	$(RUSTC) main.rs --crate-type bin -Crpath \
		--target $(TARGET) $(RUSTFLAGS) --print link-args | $(CGREP) -rpath

# ferrocene-annotations: um_rustc_C_rpath
