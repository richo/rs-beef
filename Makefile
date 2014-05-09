RUSTC := $(shell which rustc)

all: beef

beef: beef.rs
	$(RUSTC) -o $@ $<
