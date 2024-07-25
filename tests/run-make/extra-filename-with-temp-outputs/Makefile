<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
# ignore-cross-compile
include ../tools.mk

all:
	$(RUSTC) -C extra-filename=bar foo.rs -C save-temps
	rm $(TMPDIR)/foobar.foo*0.rcgu.o
	rm $(TMPDIR)/$(call BIN,foobar)

# ferrocene-annotations: um_rustc_C_extra_filename
