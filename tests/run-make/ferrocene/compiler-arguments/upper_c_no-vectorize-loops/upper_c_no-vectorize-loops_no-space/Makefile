# As other codegen options, no-vectorize-loops should also work without a space
# after -C
include ../../../../../run-make-fulldeps/tools.mk

all:
	$(RUSTC) main.rs --crate-type bin --target $(TARGET) $(RUSTFLAGS) \
	-Cno-vectorize-loops

#ferrocene-annotations: um_rustc_C_no_vectorize_loops
