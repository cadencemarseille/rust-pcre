.PHONY: lib install test

lib:
	rustpkg build pcre

install:
	rustpkg install pcre

test:
	rust test src/pcre/lib.rs
