// Copyright 2014 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(unused_must_use)]
#![feature(phase)]

#[phase(plugin, link)] extern crate log;
extern crate rustc;

use rustc::driver::driver::host_triple;
use std::from_str::{from_str};
use std::io;
use std::io::{Command, FilePermission};
use std::io::fs::{mkdir, File};
use std::option::{Option};
use std::os;
use std::str;
use std::string;

#[deriving(Eq, PartialEq)]
struct Version {
    major: uint,
    minor: uint
}

impl Version {
    pub fn parse(version_str: &str) -> Option<Version> {
        let mut it = version_str.split('.');
        match (it.next().and_then(from_str::<uint>), it.next().and_then(from_str::<uint>)) {
            (Some(major), Some(minor)) => Some(Version { major: major, minor: minor }),
            _                          => None
        }
    }
}

impl std::fmt::Show for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:u}.{:u}", self.major, self.minor)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        (self.major, self.minor).cmp(&(other.major, other.minor))
    }
}

fn cd(path: &Path) {
    if !os::change_dir(path) {
        fail!("Package script error: Failed to `cd` into `{}`", path.display());
    }
}

fn main() {
    let pcre_libdir = match os::getenv("PCRE_LIBDIR") {
        None            => {
            let pcre_config_output = match Command::new("pcre-config").arg("--prefix").output() {
                Err(e) => {
                    match e.kind {
                        io::FileNotFound => fail!("Package script error: Could not run `pcre-config` because no such executable is in the executable search PATH. Make sure that you have installed a dev package for libpcre and/or make sure that libpcre's bindir is added to your PATH (currently \"{}\").", os::getenv("PATH").unwrap_or(String::from_str(""))),
                        _ => fail!("Package script error: Could not run `pcre-config`: {}", e)
                    }
                },
                Ok(pcre_config_output) => pcre_config_output
            };
            if !pcre_config_output.status.success() {
                fail!("Package script error: `pcre-config --prefix` failed");
            }
            let output_ptr = pcre_config_output.output.as_ptr();
            let output_len = pcre_config_output.output.len();
            let prefix_str = unsafe { string::raw::from_buf_len(output_ptr, output_len) };
            // `pcre-config` adds a newline to the end, which we need to trim away.
            String::from_str(prefix_str.as_slice().trim()).append("/lib")
        },
        Some(pcre_libdir) => pcre_libdir
    };
    let pcre_lib_path = Path::new(pcre_libdir);

    let workspace_path = os::getcwd();

    // Check the version
    let target_build_path = workspace_path.join("build").join(host_triple());
    if !target_build_path.exists() {
        if mkdir(&target_build_path, FilePermission::from_bits_truncate(0x1FF)).is_err() {
            fail!("Package script error: Failed to create target build directory `{}`", target_build_path.display());
        }
    }
    let out_path = target_build_path.join("pcre");
    if !out_path.exists() {
        if mkdir(&out_path, FilePermission::from_bits_truncate(0x1FF)).is_err() {
            fail!("Package script error: Failed to create output directory `{}`", out_path.display());
        }
    }

    let versioncheck_rs_path = out_path.join("versioncheck.rs");
    {
        let mut f = match File::create(&versioncheck_rs_path) {
            Err(e) => fail!("Package script error: Failed to open `{}` for writing: {}", versioncheck_rs_path.display(), e),
            Ok(f) => f
        };
        let contents = format!("\
extern crate libc;

use std::c_str::{{CString}};
use libc::{{c_char, c_int, c_uchar, c_void}};
use std::ptr;
use std::ptr::{{RawPtr}};
use std::slice;

type options = c_int;
struct pcre;
struct pcre_extra;

#[link(name = \"pcre\")]
extern {{
    static pcre_free: extern \"C\" fn(ptr: *const c_void);

    fn pcre_compile(pattern: *const c_char, options: options, errptr: *mut *const c_char, erroffset: *mut c_int, tableptr: *const c_uchar) -> *const pcre;
    fn pcre_exec(code: *const pcre, extra: *const pcre_extra, subject: *const c_char, length: c_int, startoffset: c_int, options: options, ovector: *mut c_int, ovecsize: c_int) -> c_int;
    fn pcre_version() -> *const c_char;
}}

fn main () {{
    unsafe {{
        let version_cstring = CString::new(pcre_version(), false);
        let version_str = version_cstring.as_str().unwrap().to_string();

        let pattern = \"^\\\\d+\\\\.\\\\d+\";
        pattern.with_c_str(|pattern_c_str| {{
            let mut err: *const c_char = ptr::null();
            let mut erroffset: c_int = 0;
            let code = pcre_compile(pattern_c_str, 0, &mut err, &mut erroffset, ptr::null());
            if code.is_null() {{
                if code.is_null() {{
                    let err_cstring = CString::new(err, false);
                    match err_cstring.as_str() {{
                        None          => fail!(\"pcre_compile() failed at offset {{}}\", erroffset as uint),
                        Some(err_str) => fail!(\"pcre_compile() failed at offset {{}}: {{}}\", erroffset as uint, err_str)
                    }}
                }}
            }}
            assert!(code.is_not_null());

            let ovecsize = 1 * 3;
            let mut ovector = Vec::from_elem(ovecsize, 0 as c_int);
            version_str.with_c_str_unchecked(|version_c_str| {{
                let rc = pcre_exec(code, ptr::null(), version_c_str, version_str.len() as c_int, 0, 0, ovector.as_mut_ptr(), ovecsize as c_int);
                if rc < 0 {{
                    fail!(\"pcre_exec() failed\");
                }}

                print!(\"{{}}\", version_str.as_slice().slice_to(*ovector.get(1) as uint));
            }});

            pcre_free(code as *const c_void);
        }});
    }}
}}
");
        f.write_str(contents.as_slice()).map_err(|e| -> () {
            fail!("Package script error: Failed to write to `{}`: {}", versioncheck_rs_path.display(), e);
        });
    }

    // Compile and run `versioncheck.rs`
    cd(&out_path);
    let rustc_output = match Command::new("rustc").arg("versioncheck.rs").arg("-L").arg(pcre_lib_path.clone()).output() {
        Err(e) => fail!("Package script error: Failed to run `rustc`: {}", e),
        Ok(rustc_output) => rustc_output
    };
    if !rustc_output.status.success() {
        println!("{}", str::from_utf8(rustc_output.output.as_slice()));
        println!("{}", str::from_utf8(rustc_output.error.as_slice()));
        fail!("Package script error: `rustc versioncheck.rs` failed: {}", rustc_output.status);
    }
    let versioncheck_output = match Command::new("./versioncheck").output() {
        Err(e) => fail!("Package script error: Failed to run `./versioncheck`: {}", e),
        Ok(versioncheck_output) => versioncheck_output
    };
    if !versioncheck_output.status.success() {
        println!("{}", str::from_utf8(versioncheck_output.output.as_slice()));
        println!("{}", str::from_utf8(versioncheck_output.error.as_slice()));
        fail!("versioncheck error: {}", versioncheck_output.status);
    }
    cd(&workspace_path);

    let output_ptr = versioncheck_output.output.as_ptr();
    let output_len = versioncheck_output.output.len();
    let output_str = unsafe { string::raw::from_buf_len(output_ptr, output_len) };
    debug!("output_str = `{}`", output_str);

    // The "no debug symbols in executable" warning may be present in the output.
    // https://github.com/mozilla/rust/issues/3495
    let mut output_rsplit_iter = output_str.as_slice().split('\n').rev();
    let version_str: String = match output_rsplit_iter.next() {
        None              => output_str.clone(),
        Some(version_str) => version_str.to_string()
    };

    debug!("libpcre version {:s}", version_str.as_slice());

    let min_required_version = Version::parse("8.20").unwrap();
    let pcre_version = match Version::parse(version_str.as_slice()) {
        None               => fail!("Package script error: Failed to parse version string '{}'", version_str.as_slice()),
        Some(pcre_version) => pcre_version
    };

    if pcre_version < min_required_version {
        fail!("Package script error: Found libpcre version {}, but at least version {} is required", version_str.as_slice(), min_required_version);
    }

    // Create directories `bin` and `lib`
    let bin_path = workspace_path.join("bin");
    if !bin_path.exists() {
        if mkdir(&bin_path, FilePermission::from_bits_truncate(0x1FF)).is_err() {
            fail!("Package script error: Failed to create the `bin` directory");
        }
    }
    let lib_path = workspace_path.join("lib");
    if !lib_path.exists() {
        if mkdir(&lib_path, FilePermission::from_bits_truncate(0x1FF)).is_err() {
            fail!("Package script error: Failed to create the `lib` directory");
        }
    }

    // Compile libpcre-*.rlib
    match Command::new("rustc").arg("--out-dir").arg(lib_path).arg("src/pcre/mod.rs").arg("-L").arg(pcre_lib_path.clone()).output() {
        Err(e) => fail!("Package script error: Failed to run `rustc`: {}", e),
        Ok(rustc_output) => {
            if !rustc_output.status.success() {
                println!("{}", str::from_utf8(rustc_output.output.as_slice()));
                println!("{}", str::from_utf8(rustc_output.error.as_slice()));
                fail!("Package script error: `rustc src/pcre/mod.rs` failed: {}", rustc_output.status);
            }
        }
    }

    match Command::new("rustc").arg("-o").arg(bin_path.join("pcredemo")).arg("src/pcredemo/main.rs").arg("-L").arg("lib").arg("-L").arg(pcre_lib_path.clone()).output() {
        Err(e) => fail!("Package script error: Failed to run `rustc`: {}", e),
        Ok(rustc_output) => {
            if !rustc_output.status.success() {
                println!("{}", str::from_utf8(rustc_output.output.as_slice()));
                println!("{}", str::from_utf8(rustc_output.error.as_slice()));
                fail!("Package script error: `rustc src/pcredemo/main.rs` failed: {}", rustc_output.status);
            }
        }
    }
}
