<<<PULL-UPSTREAM>>> file deleted upstream; fix the conflict and remove this line
# ignore-cross-compile
include ../tools.mk

all:
	$(RUSTC) both.rs -C prefer-dynamic
	$(RUSTC) dylib.rs -C prefer-dynamic
	$(RUSTC) prog.rs
	$(call RUN,prog)

# ferrocene-annotations: um_rustc_C_prefer_dynamic
