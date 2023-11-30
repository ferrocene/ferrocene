include ../../../../../run-make-fulldeps/tools.mk

RUSTC_FLAGS = -C link-args="-lfoo -lboo" -C link-args="-lbar -lfar" --print link-args

all:
	$(RUSTC) $(RUSTC_FLAGS) main.rs | $(CGREP) lfoo lbar lboo lfar

# ferrocene-annotations: um_rustc_C_link_args
