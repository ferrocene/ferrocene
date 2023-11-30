include ../../../../../run-make-fulldeps/tools.mk

all:
	mkdir $(TMPDIR)/L_native_dir
	$(call COMPILE_OBJ,$(TMPDIR)/L_native_dir/liboperations.o,operations.c)
	$(AR) crus $(TMPDIR)/L_native_dir/liboperations.a $(TMPDIR)/L_native_dir/liboperations.o
	$(RUSTC) main.rs --crate-type bin -L native=$(TMPDIR)/L_native_dir -l operations --target $(TARGET) $(RUSTFLAGS)
	$(call RUN,main)

# ferrocene-annotations: um_rustc_L
