include ../tools.mk

all:
	$(RUSTC) foo.rs -Copt-level=z 2>&1
	$(call RUN,foo)

# ferrocene-annotations: um_rustc_C_opt_level
