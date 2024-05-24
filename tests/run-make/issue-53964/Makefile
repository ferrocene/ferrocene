<<<PULL-UPSTREAM>>> file deleted upstream; fix the conflict and remove this line
include ../tools.mk

all:
	$(RUSTC) panic.rs
	$(RUSTC) -C panic=abort --emit=obj app.rs -L $(TMPDIR)

# ferrocene-annotations: um_rustc_C_panic
