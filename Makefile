ifndef PCRE_LIBDIR
PCRE_LIBDIR := $(shell pcre-config --prefix)/lib
endif

.PHONY: install clean test doc

install:
	test -d build || mkdir build
	rustc --out-dir build pkg.rs && ./build/pkg

clean:
	$(RM) -r .rust bin build lib libtest~ libtest~.dSYM

test:
	test -d build || mkdir build && echo $(PCRE_LIBDIR)
	rustc --test src/pcre/test.rs -o build/libtest~ -L lib -L $(PCRE_LIBDIR) && ./build/libtest~

doc:
	rustdoc --output doc -w html src/pcre/mod.rs
