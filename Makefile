RUSTC := $(shell which rustc)

all: beef

beef: src/beef.rs
	$(RUSTC) -o $@ $<

test: beef
	./runtests.sh

clean:
	./runtests.sh --clean
	rm beef

.PHONY: test clean
