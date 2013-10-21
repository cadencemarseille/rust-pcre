.PHONY: lib install clean test doc

lib:
	rustpkg build pcre

install:
	rustpkg install pcre

clean:
	$(RM) -r .rust bin build lib libtest~ libtest~.dSYM

test:
	rustc --test src/pcre/test.rs -o libtest~ && ./libtest~

doc:
	rustdoc --output doc html src/pcre/mod.rs
