# Invalid relocation model should fail
include ../../../../tools.mk

all:
	$(RUSTC) main.rs --crate-type bin --target $(TARGET) $(RUSTFLAGS) \
	-C relocation-model= || [ ! $$? -eq 0 ]

# ferrocene-annotations: um_rustc_C_relocation_model
