include ../../../../../run-make-fulldeps/tools.mk

# only-linux
# only-x86_64

# If no linker is given then the linker flavor is used to determine the linker
# to use.
all:
	$(RUSTC) -C linker-flavor=ld --print link-args main.rs | $(CGREP) '"ld"'
	$(RUSTC) -C linker-flavor=gcc --print link-args main.rs | $(CGREP) '"cc"'

# ferrocene-annotations: um_rustc_C_link_arg
