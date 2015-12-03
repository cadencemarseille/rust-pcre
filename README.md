# rust-pcre
[Rust](https://github.com/rust-lang/rust) 1.1+ wrapper for [libpcre](http://pcre.org/) 8.20+.

[![Build Status](https://travis-ci.org/cadencemarseille/rust-pcre.svg?branch=master)](https://travis-ci.org/cadencemarseille/rust-pcre)

## Quick Start

To use rust-pcre, you can either install libpcre 8.20+ and register with pkg-config or you can let rust-pcre build libpcre from source.

### Debian

Debian Squeeze's package for libpcre is for version 8.02 of the library, which is too old. You can either install a newer version of libpcre and register it with pkg-config or just let rust-pcre automatically build libpcre from source.

On Debian Wheezy and newer, install the `libpcre3-dev` package:

    sudo apt-get install libpcre3-dev


### Fedora

Install the `pcre-devel` package.

### Mac OS X

Mac OS 10.7 ships with version 8.02 of libpcre. You can either install a newer version of libpcre and register it with pkg-config or just let rust-pcre automatically build libpcre from source.

[Homebrew](http://brew.sh/) is highly recommended for installing libpcre. With Homebrew, installing the latest versions of Rust and libpcre is as simple as:

    brew install rust pcre

To upgrade:

    brew update && brew upgrade rust pcre

### Ubuntu
The libpcre packages for Ubuntu 10.04 LTS 'Lucid Lynx' and Ubuntu 12.04 LTS 'Precise Pangolin' are too old. You can either install a newer version of libpcre and register it with pkg-config or just let rust-pcre automatically build libpcre from source.

On Ubuntu 12.10 'Quantal Quetzal' and newer, install the `libpcre3-dev` package:

    sudo apt-get install libpcre3-dev

## Usage
The basic use of the library involves compiling a pattern regular expression:

    let re = match Pcre::compile(pattern) {
        Err(err) => {
            // compilation failed
            return;
        },
        Ok(re) => re
    };

You can also pass options:

    let mut compile_options: EnumSet<CompileOption> = EnumSet::new();
    compile_options.insert(CompileOption::Caseless);
    let re = Pcre::compile_with_options(pattern, &compile_options).unwrap();

To test against a subject string, use one of the exec(), exec_from(), or exec_from_with_options() methods. For example:

    let m = match re.exec(subject) {
        None => { println("No match"); return; },
        Some(m) => m
    };

See the [source of `pcredemo`](https://github.com/cadencemarseille/rust-pcre/blob/master/examples/pcredemo.rs) for a complete example.
