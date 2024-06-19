<<<PULL-UPSTREAM>>> file deleted upstream; fix the conflict and remove this line
include ../tools.mk

# ignore-windows-msvc

all:
	$(RUSTC) -C opt-level=3 --emit=obj used.rs
	nm $(TMPDIR)/used.o | $(CGREP) FOO

# ferrocene-annotations: um_rustc_C_opt_level
