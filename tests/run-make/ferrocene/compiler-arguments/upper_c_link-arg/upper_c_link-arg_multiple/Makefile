include ../../../../../run-make-fulldeps/tools.mk

RUSTC_FLAGS = -C link-arg="-lfoo" -C link-arg="-lbar" --print link-args

all:
	$(RUSTC) $(RUSTC_FLAGS) main.rs | $(CGREP) lfoo lbar

# ferrocene-annotations: um_rustc_C_link_arg
