RUSTC := $(shell which rustc)
RUST_FLAGS = -L build

all: beef

libbeef_so = build/libbeef.timestamp


$(libbeef_so): Makefile $(wildcard src/*.rs)
	mkdir -p build/
	$(RUSTC) $(RUST_FLAGS) src/lib.rs --out-dir=build
	@touch $@


beef: src/beef.rs $(libbeef_so)
	$(RUSTC) $(RUST_FLAGS) -o $@ $<

test: beef
	./runtests.sh

clean:
	./runtests.sh --clean
	rm beef

.PHONY: test clean
