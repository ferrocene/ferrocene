<<<PULL-UPSTREAM>>> file deleted upstream; fix the conflict and remove this line
# ignore-cross-compile
include ../tools.mk

all:
	$(RUSTC) bar.rs --crate-type=dylib --crate-type=rlib -C prefer-dynamic
	$(RUSTC) foo.rs -C prefer-dynamic
	$(call RUN,foo)
	rm $(TMPDIR)/*bar*
	$(call FAIL,foo)

# ferrocene-annotations: um_rustc_C_prefer_dynamic
