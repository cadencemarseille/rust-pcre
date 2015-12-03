// Copyright 2015 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate bzip2;
extern crate pkg_config;
extern crate tar;

use bzip2::reader::{BzDecompressor};
use std::env;
use std::fs::{OpenOptions};
use std::io::{ErrorKind};
use std::path::{Path};
use std::process::{Command};
use tar::{Archive};

const BUNDLED_PCRE_VERSION: &'static str = "8.38";

fn main() {
    match pkg_config::Config::new().atleast_version("8.20").find("libpcre") {
        Ok(pkg_config_lib) => {
            for link_pathbuf in pkg_config_lib.link_paths {
                println!("cargo:rustc-link-search=native={}", link_pathbuf.as_path().display());
            }
        },
        Err(_) => {
            let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            let out_dir = env::var("OUT_DIR").unwrap();

            let ext_pathbuf = Path::new(&cargo_manifest_dir).join("ext");

            let pcre_tbz2_pathbuf = ext_pathbuf.join(format!("pcre-{}.tar.bz2", BUNDLED_PCRE_VERSION));
            let pcre_tbz2_f = OpenOptions::new().read(true).open(pcre_tbz2_pathbuf).unwrap();
            let decompressor = BzDecompressor::new(pcre_tbz2_f);

            let mut archive = Archive::new(decompressor);
            if !archive.unpack(&out_dir).is_ok() {
                panic!("failed to extract the tarball");
            }

            let pcre_pathbuf = Path::new(&out_dir).join(format!("pcre-{}", BUNDLED_PCRE_VERSION));

            if cfg!(unix) {
                let mut cmd = Command::new("autoreconf");
                cmd.current_dir(&pcre_pathbuf);
                let status = match cmd.status() {
                    Err(ref e) if e.kind() == ErrorKind::NotFound => {
                        panic!("failed to execute `autoreconf`: {}. Are the Autotools installed?", e);
                    },
                    Err(e) => {
                        panic!("failed to execute `autoreconf`: {}", e);
                    },
                    Ok(status) => status
                };
                if !status.success() {
                    panic!("`autoreconf` did not run successfully.");
                }

                let mut cmd = Command::new("./configure");
                cmd.arg("--with-pic");
                cmd.arg("--disable-shared");
                cmd.arg("--disable-cpp");
                cmd.arg("--enable-jit");
                cmd.arg("--enable-utf");
                cmd.arg("--enable-unicode-properties");
                cmd.arg(format!("--prefix={}", Path::new(&out_dir).display()));
                cmd.current_dir(&pcre_pathbuf);
                let status = match cmd.status() {
                    Err(e) => {
                        panic!("failed to execute `./configure`: {}", e);
                    },
                    Ok(status) => status
                };
                if !status.success() {
                    panic!("`./configure --with-pic ...` did not run successfully.");
                }

                let mut cmd = Command::new("make");
                cmd.arg("install");
                cmd.current_dir(&pcre_pathbuf);
                let status = match cmd.status() {
                    Err(ref e) if e.kind() == ErrorKind::NotFound => {
                        panic!("failed to execute `make`: {}. Is GNU Make installed?", e);
                    },
                    Err(e) => {
                        panic!("failed to execute `make`: {}", e);
                    },
                    Ok(status) => status
                };
                if !status.success() {
                    panic!("`make install` did not run successfully.");
                }

                println!("cargo:rustc-link-search=native={}", Path::new(&out_dir).join("lib").as_path().display());
            } else {
                let mut cmd = Command::new("cmake");
                cmd.arg(".");
                cmd.arg("-DBUILD_SHARED_LIBS=OFF");
                cmd.arg("-DPCRE_BUILD_PCRECPP=OFF");
                cmd.arg("-DPCRE_BUILD_PCREGREP=OFF");
                cmd.arg("-DPCRE_BUILD_TESTS=OFF");
                cmd.arg("-DPCRE_BUILD_PCRE8=ON");
                cmd.arg("-DPCRE_SUPPORT_JIT=ON");
                cmd.arg("-DPCRE_SUPPORT_UTF=ON");
                cmd.arg("-DPCRE_SUPPORT_UNICODE_PROPERTIES=ON");
                cmd.current_dir(&pcre_pathbuf);
                let status = match cmd.status() {
                    Err(ref e) if e.kind() == ErrorKind::NotFound => {
                        panic!("failed to execute `cmake`: {}. Is CMake installed?", e);
                    },
                    Err(e) => {
                        panic!("failed to execute `cmake`: {}", e);
                    },
                    Ok(status) => status
                };
                if !status.success() {
                    panic!("`cmake . -DBUILD_SHARED_LIBS=OFF ...` did not run successfully.");
                }

                let mut cmd = Command::new("cmake");
                cmd.arg("--build").arg(".").current_dir(&pcre_pathbuf);
                let status = match cmd.status() {
                    Err(ref e) if e.kind() == ErrorKind::NotFound => {
                        panic!("failed to execute `cmake`: {}. Is CMake installed?", e);
                    },
                    Err(e) => {
                        panic!("failed to execute `cmake`: {}", e);
                    },
                    Ok(status) => status
                };
                if !status.success() {
                    panic!("`cmake --build .` did not run successfully.");
                }

                println!("cargo:rustc-link-search=native={}", pcre_pathbuf.as_path().display());
            }
        }
    };
}
