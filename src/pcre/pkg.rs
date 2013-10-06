// Copyright 2013 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern mod extra;
extern mod rustc;
extern mod rustpkg;

use extra::time;
use rustc::driver::driver::host_triple;
use rustpkg::api;
use std::from_str::{from_str};
use std::io;
use std::option::{Option};
use std::os;
use std::run;
use std::str;
use std::util;
use std::vec;

struct Version {
    major: uint,
    minor: uint
}

impl Version {
    pub fn parse(version_str: &str) -> Option<Version> {
        let mut it = version_str.split_iter('.');
        match (it.next().and_then(from_str::<uint>), it.next().and_then(from_str::<uint>)) {
            (Some(major), Some(minor)) => Some(Version { major: major, minor: minor }),
            _                          => None
        }
    }
}

impl ToStr for Version {
    fn to_str(&self) -> ~str {
        fmt!("%u.%u", self.major, self.minor)
    }
}

impl Ord for Version {
    fn ge(&self, other: &Version) -> bool {
        !self.lt(other)
    }

    fn gt(&self, other: &Version) -> bool {
        self.major > other.major || (self.major == other.major && self.minor > other.minor)
    }

    fn le(&self, other: &Version) -> bool {
        !self.gt(other)
    }

    fn lt(&self, other: &Version) -> bool {
        self.major < other.major || (self.major == other.major && self.minor < other.minor)
    }
}

fn cd(path: &Path) {
    if !os::change_dir(path) {
        fail2!("Package script error: Failed to `cd` into `{}`", path.to_str());
    }
}

fn do_install(args: ~[~str]) {
    let sysroot_arg = args[1].clone();
    let sysroot_path = Path(sysroot_arg);

    let pcre_libs = match os::getenv("PCRE_LIBS") {
        None            => {
            let pcre_config_output = run::process_output("pcre-config", [~"--libs"]);
            if pcre_config_output.status != 0 {
                fail!("Package script error: `pcre-config` failed");
            }
            let output_ptr = vec::raw::to_ptr(pcre_config_output.output);
            let output_len = pcre_config_output.output.len();
            let libs_str = unsafe { str::raw::from_buf_len(output_ptr, output_len) };
            os::setenv("PCRE_LIBS", libs_str);
            libs_str
        },
        Some(pcre_libs) => pcre_libs
    };
    // `pcre-config` adds a newline to the end, which we need to trim away because newlines
    // in link_args cause build issues.
    let trimmed_pcre_libs = pcre_libs.trim();
    debug!("PCRE_LIBS=\"%s\"", trimmed_pcre_libs);

    let workspace_path = os::getcwd();

    // Check the version
    let target_build_path = workspace_path.push("build").push(host_triple());
    if !os::path_exists(&target_build_path) {
        if !os::make_dir(&target_build_path, 0x1FF) {
            fail2!("Package script error: Failed to create target build directory `{}`", target_build_path.to_str());
        }
    }
    let out_path = target_build_path.push("pcre");
    if !os::path_exists(&out_path) {
        if !os::make_dir(&out_path, 0x1FF) {
            fail2!("Package script error: Failed to create output directory `{}`", out_path.to_str());
        }
    }

    let versioncheck_rs_path = out_path.push("versioncheck.rs");
    {
        let w = match io::file_writer(&versioncheck_rs_path, [io::Create]) {
            Err(err_str) => fail2!("Package script error: Failed to open `{}` for writing: {}", versioncheck_rs_path.to_str(), err_str),
            Ok(w)        => w
        };
        w.write_str("\
use std::c_str::{CString};
use std::libc::*;
use std::ptr;
use std::vec;

type options = c_int;
struct pcre;
struct pcre_extra;

#[link_args = \"" + trimmed_pcre_libs + "\"]
extern {
    static pcre_free: extern \"C\" fn(ptr: *c_void);

    fn pcre_compile(pattern: *c_char, options: options, errptr: *mut *c_char, erroffset: *mut c_int, tableptr: *c_uchar) -> *pcre;
    fn pcre_exec(code: *pcre, extra: *pcre_extra, subject: *c_char, length: c_int, startoffset: c_int, options: options, ovector: *mut c_int, ovecsize: c_int) -> c_int;
    fn pcre_version() -> *c_char;
}

#[fixed_stack_segment]
#[inline(never)]
fn main () {
    unsafe {
        let version_cstring = CString::new(pcre_version(), false);
        let version_str = version_cstring.as_str().unwrap().to_owned();

        let pattern = \"^\\\\d+\\\\.\\\\d+\";
        do pattern.with_c_str |pattern_c_str| {
            let mut err: *c_char = ptr::null();
            let mut erroffset: c_int = 0;
            let code = pcre_compile(pattern_c_str, 0, &mut err, &mut erroffset, ptr::null());
            if ptr::is_null(code) {
                if ptr::is_null(code) {
                    let err_cstring = CString::new(err, false);
                    match err_cstring.as_str() {
                        None          => fail2!(\"pcre_compile() failed at offset {}\", erroffset as uint),
                        Some(err_str) => fail2!(\"pcre_compile() failed at offset {}: {}\", erroffset as uint, err_str)
                    }
                }
            }
            assert!(ptr::is_not_null(code));

            let ovecsize = 1 * 3;
            let mut ovector: ~[c_int] = vec::from_elem(ovecsize, 0 as c_int);
            do version_str.with_c_str_unchecked |version_c_str| {
                let rc = pcre_exec(code, ptr::null(), version_c_str, version_str.len() as c_int, 0, 0, vec::raw::to_mut_ptr(ovector), ovecsize as c_int);
                if rc < 0 {
                    fail!(\"pcre_exec() failed\");
                }

                print(version_str.slice_to(ovector[1] as uint));
            }

            pcre_free(code as *c_void);
        }
    }
}
");
    }

    // Compile and run `versioncheck.rs`
    cd(&out_path);
    let rust_run_output = run::process_output("rust", [~"run", ~"versioncheck.rs"]);
    cd(&workspace_path);
    if rust_run_output.status != 0 {
        fail!("Package script error: `rust run versioncheck.rs` failed");
    }
    let output_ptr = vec::raw::to_ptr(rust_run_output.output);
    let output_len = rust_run_output.output.len();
    let output_str = unsafe { str::raw::from_buf_len(output_ptr, output_len) };

    // The "no debug symbols in executable" warning may be present in the output.
    // https://github.com/mozilla/rust/issues/3495
    let mut output_rsplit_iter = output_str.rsplit_iter('\n');
    let version_str: ~str = match output_rsplit_iter.next() {
        None              => output_str.clone(),
        Some(version_str) => version_str.to_owned()
    };

    debug!("libpcre version %s", version_str);

    let min_required_version = Version::parse("8.20").unwrap();
    let pcre_version = match Version::parse(version_str) {
        None               => fail2!("Package script error: Failed to parse version string '{}'", version_str),
        Some(pcre_version) => pcre_version
    };
    if pcre_version < min_required_version {
        fail2!("Package script error: Found libpcre version {}, but at least version {} is required", version_str, min_required_version.to_str());
    }

    let src_path = workspace_path.push("src");

    // Output `src/pcre/detail/native.rs`
    let detail_src_path = src_path.push("pcre").push("detail");
    if !os::path_exists(&detail_src_path) {
        fail2!("Package script error: Source directory `{}` does not exist.", detail_src_path.to_str());
    }
    let native_rs_path = detail_src_path.push("native.rs");
    let native_rs_in_path = detail_src_path.push("native.rs.in");
    if !os::path_exists(&native_rs_in_path) {
        fail2!("Package script error: Source file `{}` does not exist.", native_rs_in_path.to_str());
    }
    {
        let r = match io::file_reader(&native_rs_in_path) {
            Err(err_str) => fail2!("Package script error: Failed to open `{}` for reading: {}", native_rs_in_path.to_str(), err_str),
            Ok(r)        => r
        };
        let w = match io::file_writer(&native_rs_path, [io::Create]) {
            Err(err_str) => fail2!("Package script error: Failed to open `{}` for writing: {}", native_rs_path.to_str(), err_str),
            Ok(w)        => w
        };
        w.write_line("// -*- buffer-read-only: t -*-");
        w.write_line("// Generated by " + args[0] + " on " + time::now().rfc822());
        w.write_char('\n');

        do r.each_line |line| -> bool {
            let substituted_line = str::replace(line, "@PCRE_LIBS@", trimmed_pcre_libs);
            w.write_line(substituted_line);
            true
        };
    }

    api::build_lib(sysroot_path, workspace_path, ~"pcre", rustpkg::version::ExactRevision(~"0.1"), Path("mod.rs"));
}

fn do_configs(args: ~[~str]) {
    util::ignore(args);
}

fn main() {
    let args = os::args();
    let args_len = args.len();

    if args_len < 2 {
        fail!("Package script requires a directory where rustc libraries live as the first argument");
    } else if args_len < 3 {
        fail!("Package script requires a command as the second argument");
    }

    if args[2] == ~"install" {
        do_install(args);
    } else if args[2] == ~"configs" {
        do_configs(args);
    } else {
        fail2!("Package script error: Unsupported command `{}`", args[2]);
    }
}
