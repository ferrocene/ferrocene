include ../tools.mk

# ignore-windows

# Notice the space in the end, this emulates the output of pkg-config
RUSTC_FLAGS = -C link-args="-lc "

all:
	$(RUSTC) $(RUSTC_FLAGS) empty.rs

# ferrocene-annotations: um_rustc_C_link_args
