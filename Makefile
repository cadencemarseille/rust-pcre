.PHONY: install clean test doc

install:
	rustpkg install pcre
	test -d bin || mkdir bin
	rustc src/pcredemo/main.rs -o bin/pcredemo

clean:
	$(RM) -r .rust bin build lib libtest~ libtest~.dSYM

test:
	rustc --test src/pcre/test.rs -o libtest~ && ./libtest~

doc:
	rustdoc --output doc -w html src/pcre/mod.rs
