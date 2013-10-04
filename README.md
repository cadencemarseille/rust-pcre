# rust-pcre
[Rust](https://github.com/mozilla/rust) 0.8+ wrapper for [libpcre](http://pcre.org/) 8.20+.

## Quick Start

### General setup
If you have not done so already, you need to set up your rustpkg path:

    test ! -d ~/.rust && mkdir ~/.rust
    cd ~/.rust
    rustpkg init

### Mac OS X

Mac OS 10.7 ships with version 8.02 of libpcre, so you'll need to install a newer version of the pcre library.

[Homebrew](http://brew.sh/) is highly recommended for installing this project's dependencies. With Homebrew, installing the latest versions of Rust and libpcre is as simple as:

    brew install rust pcre

To upgrade:

    brew update && brew upgrade rust pcre

With Rust and libpcre 8.20+ installed:

    make install

## Development

Patches and GitHub pull requests (PRs) are always welcome.

To run the tests:

    make test
