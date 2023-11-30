include ../../../../../run-make-fulldeps/tools.mk

# From rustc documentation:
#
# This option allows you to control the metadata used for symbol mangling. This
# takes a space-separated list of values. Mangled symbols will incorporate a
# hash of the metadata. This may be used, for example, to differentiate symbols
# between two different versions of the same crate being linked.
#
# What we test:
#
# Ensure files with same metadata are equal and files with different metadata
# are different.
#
# Other non-documented rules, which are clear by looking at the code in charge
# of generating a stable crate id:
#
# Metadata is a list of values. The order of such values does not matter.
# So ["one, two"] and ["two", "one"] are equivalent
#
# Metadata values can be added with a single -Cmetadata option, which takes
# a string of space-separated values.
#
# Metadata values can also be added by multiple -Cmetadata options.
#
# Every distinct -C metadata value is only incorporated once
#
# So these are equivalent:
#
# -Cmetadata=one -Cmetadata=two
# -Cmetadata="one two"
# -Cmetadata="two one"
# -Cmetadata="one two one"
#
# Only space is a valid delimiter. Unlike other codegen options, comma is
# treated as any other character.
#
# A single metadata "ab" has a different meaning than two separated "a" "b"

all:
	$(RUSTC) --target $(TARGET) $(RUSTC_FLAGS) main.rs -Cmetadata="one" -Cextra-filename=-one
	$(RUSTC) --target $(TARGET) $(RUSTC_FLAGS) main.rs -Cmetadata="one,two" -Cextra-filename=-one,two
	$(RUSTC) --target $(TARGET) $(RUSTC_FLAGS) main.rs -Cmetadata="onetwo" -Cextra-filename=-onetwo
	$(RUSTC) --target $(TARGET) $(RUSTC_FLAGS) main.rs -Cmetadata="one two" -Cextra-filename=-one-two
	$(RUSTC) --target $(TARGET) $(RUSTC_FLAGS) main.rs -Cmetadata="one" -Cmetadata="two" -Cextra-filename=-one--two
	$(RUSTC) --target $(TARGET) $(RUSTC_FLAGS) main.rs -Cmetadata="two one" -Cextra-filename=-two-one
	$(RUSTC) --target $(TARGET) $(RUSTC_FLAGS) main.rs -Cmetadata="two" -Cmetadata="one" -Cextra-filename=-two--one
	$(RUSTC) --target $(TARGET) $(RUSTC_FLAGS) main.rs -Cmetadata="one two" -Cmetadata="one" -Cextra-filename=-one-two--one
	diff $(TMPDIR)/main-one-two $(TMPDIR)/main-one--two
	diff $(TMPDIR)/main-one-two $(TMPDIR)/main-two-one
	diff $(TMPDIR)/main-one-two $(TMPDIR)/main-two--one
	diff $(TMPDIR)/main-one-two $(TMPDIR)/main-one-two--one
	if diff $(TMPDIR)/main-one $(TMPDIR)/main-one-two; then exit 1; fi
	if diff $(TMPDIR)/main-one-two $(TMPDIR)/main-one,two; then exit 1; fi
	if diff $(TMPDIR)/main-one-two $(TMPDIR)/main-onetwo; then exit 1; fi

# ferrocene-annotations: um_rustc_C_metadata
