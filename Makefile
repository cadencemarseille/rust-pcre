.PHONY: lib install test

lib:
	rustpkg build pcre

install:
	rustpkg install pcre

test:
	rustc --test src/pcre/test.rs -o libtest~ && ./libtest~
