include ../tools.mk
RUSTC_FLAGS = -C link-arg="-lfoo" -C link-arg="-lbar" --print link-args

all:
	$(RUSTC) $(RUSTC_FLAGS) empty.rs | $(CGREP) lfoo lbar

# ferrocene-annotations: um_rustc_C_link_arg
