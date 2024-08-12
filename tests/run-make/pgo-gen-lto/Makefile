<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
# needs-profiler-support
# ignore-cross-compile

include ../tools.mk

COMPILE_FLAGS=-Copt-level=3 -Clto=fat -Cprofile-generate="$(TMPDIR)" --target $(TARGET)

all:
	$(RUSTC) $(COMPILE_FLAGS) test.rs
	$(call RUN,test) || exit 1
	[ -e "$(TMPDIR)"/default_*.profraw ] || (echo "No .profraw file"; exit 1)

# ferrocene-annotations: um_rustc_C_opt_level
