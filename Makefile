.PHONY: lib install test doc

lib:
	rustpkg build pcre

install:
	rustpkg install pcre

test:
	rustc --test src/pcre/test.rs -o libtest~ && ./libtest~

doc:
	rustdoc --output doc html src/pcre/mod.rs
