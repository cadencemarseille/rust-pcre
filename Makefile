ifndef PCRE_LIBDIR
PCRE_LIBDIR := $(shell pcre-config --prefix)/lib
endif

.PHONY: install clean test doc

install:
	test -d build || mkdir build
	rustc --out-dir build pkg.rs && ./build/pkg

clean:
	$(RM) -r .rust bin build lib libtest~ libtest~.dSYM

# I removed the -L $(PCRE_LIBDIR) because it was pulling in old versions of rust libraries on my machine 
# I didn't think it was very necessary though since rust will pull in the needed packages and pkg.rs gets libpcre
test:
	test -d build || mkdir build # && echo $(PCRE_LIBDIR)
	rustc --test src/pcre/test.rs -o build/libtest~ -L lib && ./build/libtest~

doc:
	rustdoc --output doc -w html src/pcre/mod.rs
