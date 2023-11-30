include ../tools.mk

all:
	$(RUSTC) -C extra-filename=bar foo.rs -C save-temps
	rm $(TMPDIR)/foobar.foo*0.rcgu.o
	rm $(TMPDIR)/$(call BIN,foobar)

# ferrocene-annotations: um_rustc_C_extra_filename
