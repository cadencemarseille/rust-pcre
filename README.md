# rust-pcre
[Rust](https://github.com/mozilla/rust) 0.8+ wrapper for [libpcre](http://pcre.org/) 8.20+.

## Quick Start

### Mac OS X

Mac OS 10.7 ships with version 8.02 of libpcre, so you'll need to install a newer version of the pcre library.

[Homebrew](http://brew.sh/) is highly recommended for installing this project's dependencies. With Homebrew, installing the latest versions of Rust and libpcre is as simple as:

    brew install rust pcre

To upgrade:

    brew update && brew upgrade rust pcre

With Rust and libpcre 8.20+ installed, the library is compiled with:

    rustc src/pcre.rs

## Development

Patches and GitHub pull requests (PRs) are always welcome.

To run the tests:

    rust test src/pcre.rs
