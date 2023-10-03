# Valid relocation model should succeed
include ../../../../tools.mk

all:
	$(RUSTC) main.rs --crate-type bin --target $(TARGET) $(RUSTFLAGS) \
	-C relocation-model=pie

# ferrocene-annotations: um_rustc_C_relocation_model
