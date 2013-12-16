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
use std::io::fs::{mkdir, File};
use std::io::buffered::BufferedReader;
use std::io;
use std::option::{Option};
use std::os;
use std::run;
use std::str;

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

impl ToStr for Version {
    fn to_str(&self) -> ~str {
        format!("{:u}.{:u}", self.major, self.minor)
    }
}

impl Ord for Version {
    fn lt(&self, other: &Version) -> bool {
        self.major < other.major || (self.major == other.major && self.minor < other.minor)
    }
}

fn cd(path: &Path) {
    if !os::change_dir(path) {
        fail!("Package script error: Failed to `cd` into `{}`", path.display());
    }
}

fn do_install(args: ~[~str]) {
    let sysroot_arg = args[1].clone();
    let sysroot_path = from_str(sysroot_arg).unwrap();

    let pcre_libs = match os::getenv("PCRE_LIBS") {
        None            => {
            let pcre_config_output = io::io_error::cond.trap(|e: io::IoError| {
                match e.kind {
                    io::FileNotFound => fail!("Package script error: Could not run `pcre-config` because no such executable is in the executable search PATH. Make sure that you have installed a dev package for libpcre and/or make sure that libpcre's bindir is added to your PATH (currently \"{}\").", os::getenv("PATH").unwrap_or(~"")),
                    _ => fail!("Package script error: Could not run `pcre-config`: {}", e.to_str())
                }
            }).inside(|| -> run::ProcessOutput {
                run::process_output("pcre-config", [~"--libs"]).expect("failed to exec `pcre-config`")
            });
            if !pcre_config_output.status.success() {
                fail!("Package script error: `pcre-config` failed");
            }
            let output_ptr = pcre_config_output.output.as_ptr();
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
    debug!("PCRE_LIBS=\"{:s}\"", trimmed_pcre_libs);

    let workspace_path = os::getcwd();

    // Check the version
    let target_build_path = workspace_path.join("build").join(host_triple());
    if !target_build_path.exists() {
        if !io::result(|| mkdir(&target_build_path, 0x1FF)).is_ok() {
            fail!("Package script error: Failed to create target build directory `{}`", target_build_path.display());
        }
    }
    let out_path = target_build_path.join("pcre");
    if !out_path.exists() {
        if !io::result(|| mkdir(&out_path, 0x1FF)).is_ok() {
            fail!("Package script error: Failed to create output directory `{}`", out_path.display());
        }
    }

    let versioncheck_rs_path = out_path.join("versioncheck.rs");
    {
        let mut w = match File::create(&versioncheck_rs_path) {
            None    => fail!("Package script error: Failed to open `{}` for writing", versioncheck_rs_path.display()),
            Some(w) => w
        };
        write!(&mut w as &mut Writer, "\
\\#[feature(globs)];
\\#[feature(link_args)];
use std::c_str::\\{CString\\};
use std::libc::*;
use std::ptr;
use std::vec;

type options = c_int;
struct pcre;
struct pcre_extra;

\\#[link_args = \"{:s}\"]
extern \\{
    static pcre_free: extern \"C\" fn(ptr: *c_void);

    fn pcre_compile(pattern: *c_char, options: options, errptr: *mut *c_char, erroffset: *mut c_int, tableptr: *c_uchar) -> *pcre;
    fn pcre_exec(code: *pcre, extra: *pcre_extra, subject: *c_char, length: c_int, startoffset: c_int, options: options, ovector: *mut c_int, ovecsize: c_int) -> c_int;
    fn pcre_version() -> *c_char;
\\}

fn main () \\{
    unsafe \\{
        let version_cstring = CString::new(pcre_version(), false);
        let version_str = version_cstring.as_str().unwrap().to_owned();

        let pattern = \"^\\\\\\\\d+\\\\\\\\.\\\\\\\\d+\";
        pattern.with_c_str(|pattern_c_str| \\{
            let mut err: *c_char = ptr::null();
            let mut erroffset: c_int = 0;
            let code = pcre_compile(pattern_c_str, 0, &mut err, &mut erroffset, ptr::null());
            if ptr::is_null(code) \\{
                if ptr::is_null(code) \\{
                    let err_cstring = CString::new(err, false);
                    match err_cstring.as_str() \\{
                        None          => fail!(\"pcre_compile() failed at offset \\{\\}\", erroffset as uint),
                        Some(err_str) => fail!(\"pcre_compile() failed at offset \\{\\}: \\{\\}\", erroffset as uint, err_str)
                    \\}
                \\}
            \\}
            assert!(ptr::is_not_null(code));

            let ovecsize = 1 * 3;
            let mut ovector: ~[c_int] = vec::from_elem(ovecsize, 0 as c_int);
            version_str.with_c_str_unchecked(|version_c_str| \\{
                let rc = pcre_exec(code, ptr::null(), version_c_str, version_str.len() as c_int, 0, 0, ovector.as_mut_ptr(), ovecsize as c_int);
                if rc < 0 \\{
                    fail!(\"pcre_exec() failed\");
                \\}

                print(version_str.slice_to(ovector[1] as uint));
            \\});

            pcre_free(code as *c_void);
        \\});
    \\}
\\}
", trimmed_pcre_libs);
    }

    // Compile and run `versioncheck.rs`
    cd(&out_path);
    let rustc_run_output = run::process_output("rustc", [~"versioncheck.rs"]).expect("failed to exec `rustc`");
    if !rustc_run_output.status.success() {
        println(str::from_utf8(rustc_run_output.output));
        println(str::from_utf8(rustc_run_output.error));
        fail!("Package script error: `rustc versioncheck.rs` failed: {}", rustc_run_output.status);
    }
    let version_check_output = run::process_output("./versioncheck", []).expect("failed to exec `./versioncheck`");
    if !version_check_output.status.success() {
        println(str::from_utf8(version_check_output.output));
        println(str::from_utf8(version_check_output.error));
        fail!("versioncheck error: {}", version_check_output.status);
    }
    cd(&workspace_path);

    let output_ptr = version_check_output.output.as_ptr();
    let output_len = version_check_output.output.len();
    let output_str = unsafe { str::raw::from_buf_len(output_ptr, output_len) };

    // The "no debug symbols in executable" warning may be present in the output.
    // https://github.com/mozilla/rust/issues/3495
    let mut output_rsplit_iter = output_str.rsplit('\n');
    let version_str: ~str = match output_rsplit_iter.next() {
        None              => output_str.clone(),
        Some(version_str) => version_str.to_owned()
    };

    debug!("libpcre version {:s}", version_str);

    let min_required_version = Version::parse("8.20").unwrap();
    let pcre_version = match Version::parse(version_str) {
        None               => fail!("Package script error: Failed to parse version string '{}'", version_str),
        Some(pcre_version) => pcre_version
    };

    if pcre_version < min_required_version {
        fail!("Package script error: Found libpcre version {}, but at least version {} is required", version_str, min_required_version.to_str());
    }

    let src_path = workspace_path.join("src");

    // Output `src/pcre/detail/native.rs`
    let detail_src_path = src_path.join("pcre").join("detail");
    if !detail_src_path.exists() {
        fail!("Package script error: Source directory `{}` does not exist.", detail_src_path.display());
    }
    let native_rs_path = detail_src_path.join("native.rs");
    let native_rs_in_path = detail_src_path.join("native.rs.in");
    if !native_rs_in_path.exists() {
        fail!("Package script error: Source file `{}` does not exist.", native_rs_in_path.display());
    }
    {
        let r = match File::open(&native_rs_in_path) {
            None    => fail!("Package script error: Failed to open `{}` for reading", native_rs_in_path.display()),
            Some(r) => r
        };
        let mut w = match File::create(&native_rs_path) {
            None    => fail!("Package script error: Failed to open `{}` for writing", native_rs_path.display()),
            Some(w) => w
        };
        write!(&mut w as &mut Writer, "// -*- buffer-read-only: t -*-\n");
        write!(&mut w as &mut Writer, "// Generated by {:s} on {:s}\n", args[0], time::now().rfc822());

        let mut buffered_reader = BufferedReader::new(r);
        while !buffered_reader.eof() {
            let line = buffered_reader.read_line();
            if line.is_some() {
                let substituted_line = str::replace(line.unwrap(), "@PCRE_LIBS@", trimmed_pcre_libs);
                write!(&mut w as &mut Writer, "{:s}", substituted_line);
            }
        }
    }

    api::build_lib(sysroot_path, workspace_path, ~"pcre", rustpkg::version::ExactRevision(~"0.1"), from_str("mod.rs").unwrap());
}

fn do_configs(args: ~[~str]) {
    drop(args);
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
        fail!("Package script error: Unsupported command `{}`", args[2]);
    }
}
