<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
# ignore-cross-compile
include ../tools.mk

ifeq ($(UNAME),Darwin)
FLAGS :=
else
ifdef IS_WINDOWS
FLAGS :=
else
FLAGS := -C link-args=-Wl,--no-undefined
endif
endif

all:
	$(RUSTC) bar.rs
	$(RUSTC) foo.rs $(FLAGS)
	$(RUSTC) foo.rs $(FLAGS) -C panic=abort

# ferrocene-annotations: um_rustc_C_link_args
