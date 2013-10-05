// Copyright 2013 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[link(name = "pcre", vers = "0.1")];
#[crate_type = "lib"];

use std::c_str;
use std::hashmap::{HashMap};
use std::libc::{c_char, c_int, c_uchar, c_void};
use std::option::{Option};
use std::ptr;
use std::vec;

pub type options = c_int;
pub type study_options = c_int;

pub static PCRE_CASELESS: options = 0x00000001;
pub static PCRE_MULTILINE: options = 0x00000002;
pub static PCRE_DOTALL: options = 0x00000004;
pub static PCRE_EXTENDED: options = 0x00000008;
pub static PCRE_ANCHORED: options = 0x00000010;
pub static PCRE_DOLLAR_ENDONLY: options = 0x00000020;
pub static PCRE_EXTRA: options = 0x00000040;
pub static PCRE_NOTBOL: options = 0x00000080;
pub static PCRE_NOTEOL: options = 0x00000100;
pub static PCRE_UNGREEDY: options = 0x00000200;
pub static PCRE_NOTEMPTY: options = 0x00000400;
pub static PCRE_UTF8: options = 0x00000800;
pub static PCRE_UTF16: options = 0x00000800;
pub static PCRE_UTF32: options = 0x00000800;
pub static PCRE_NO_AUTO_CAPTURE: options = 0x00001000;
pub static PCRE_NO_UTF8_CHECK: options = 0x00002000;
pub static PCRE_NO_UTF16_CHECK: options = 0x00002000;
pub static PCRE_NO_UTF32_CHECK: options = 0x00002000;
pub static PCRE_AUTO_CALLOUT: options = 0x00004000;
pub static PCRE_PARTIAL_SOFT: options = 0x00008000;
pub static PCRE_PARTIAL: options = 0x00008000;
pub static PCRE_NEVER_UTF: options = 0x00010000;
pub static PCRE_DFA_SHORTEST: options = 0x00010000;
pub static PCRE_DFA_RESTART: options = 0x00020000;
pub static PCRE_FIRSTLINE: options = 0x00040000;
pub static PCRE_DUPNAMES: options = 0x00080000;
pub static PCRE_NEWLINE_CR: options = 0x00100000;
pub static PCRE_NEWLINE_LF: options = 0x00200000;
pub static PCRE_NEWLINE_CRLF: options = 0x00300000;
pub static PCRE_NEWLINE_ANY: options = 0x00400000;
pub static PCRE_NEWLINE_ANYCRLF: options = 0x00500000;
pub static PCRE_BSR_ANYCRLF: options = 0x00800000;
pub static PCRE_BSR_UNICODE: options = 0x01000000;
pub static PCRE_JAVASCRIPT_COMPAT: options = 0x02000000;
pub static PCRE_NO_START_OPTIMIZE: options = 0x04000000;
pub static PCRE_NO_START_OPTIMISE: options = 0x04000000;
pub static PCRE_PARTIAL_HARD: options = 0x08000000;
pub static PCRE_NOTEMPTY_ATSTART: options = 0x10000000;
pub static PCRE_UCP: options = 0x20000000;

pub static PCRE_STUDY_JIT_COMPILE: study_options = 0x0001;
pub static PCRE_STUDY_JIT_PARTIAL_SOFT_COMPILE: study_options = 0x0002;
pub static PCRE_STUDY_JIT_PARTIAL_HARD_COMPILE: study_options = 0x0004;
pub static PCRE_STUDY_EXTRA_NEEDED: study_options = 0x0008;

mod detail;

pub struct Pcre {

    priv code: *detail::pcre,

    priv extra: *detail::pcre_extra,

    priv capture_count_: uint

}

pub struct Match<'self> {

    priv subject: &'self str,

    priv ovector: ~[c_int],

    priv string_count_: uint

}

impl Pcre {
    pub fn compile(pattern: &str) -> Pcre {
        Pcre::compile_with_options(pattern, 0)
    }

    pub fn compile_with_options(pattern: &str, options: options) -> Pcre {
        do pattern.with_c_str |pattern_c_str| {
            // Use the default character tables.
            let tableptr: *c_uchar = ptr::null();
            let code = detail::pcre_compile(pattern_c_str, options, tableptr);
            assert!(ptr::is_not_null(code));

            let extra: *detail::pcre_extra = ptr::null();

            let mut capture_count: c_int = 0;
            detail::pcre_fullinfo(code, extra, detail::PCRE_INFO_CAPTURECOUNT, &mut capture_count as *mut c_int as *mut c_void);

            Pcre {
                code: code,
                extra: extra,
                capture_count_: capture_count as uint
            }
        }
    }

    pub fn capture_count(&self) -> uint {
        self.capture_count_
    }

    pub fn exec<'a>(&self, subject: &'a str) -> Option<Match<'a>> {
        self.exec_from(subject, 0)
    }

    pub fn exec_from<'a>(&self, subject: &'a str, startoffset: uint) -> Option<Match<'a>> {
        self.exec_from_with_options(subject, startoffset, 0)
    }

    pub fn exec_from_with_options<'a>(&self, subject: &'a str, startoffset: uint, options: options) -> Option<Match<'a>> {
        let ovecsize = (self.capture_count_ + 1) * 3;
        let mut ovector: ~[c_int] = vec::from_elem(ovecsize, 0 as c_int);

        unsafe {
            do subject.with_c_str_unchecked |subject_c_str| -> Option<Match<'a>> {
                let rc = detail::pcre_exec(self.code, self.extra, subject_c_str, subject.len() as c_int, startoffset as c_int, options, vec::raw::to_mut_ptr(ovector), ovecsize as c_int);
                if rc >= 0 {
                    Some(Match {
                        subject: subject,
                        // TODO: Is it possible to avoid to_owned()?
                        // Probably need multiple lifetime parameters:
                        // https://mail.mozilla.org/pipermail/rust-dev/2013-September/005829.html
                        ovector: ovector.slice_to((self.capture_count_ + 1) * 2).to_owned(),
                        string_count_: rc as uint
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn name_count(&self) -> uint {
        let mut name_count: c_int = 0;
        detail::pcre_fullinfo(self.code, self.extra, detail::PCRE_INFO_NAMECOUNT, &mut name_count as *mut c_int as *mut c_void);
        name_count as uint
    }

    pub fn name_table(&self) -> HashMap<~str, ~[uint]> {
        let name_count = self.name_count();
        let mut tabptr: *c_uchar = ptr::null();
        detail::pcre_fullinfo(self.code, self.extra, detail::PCRE_INFO_NAMETABLE, &mut tabptr as *mut *c_uchar as *mut c_void);
        let mut name_entry_size: c_int = 0;
        detail::pcre_fullinfo(self.code, self.extra, detail::PCRE_INFO_NAMEENTRYSIZE, &mut name_entry_size as *mut c_int as *mut c_void);

        let mut name_table: HashMap<~str, ~[uint]> = HashMap::with_capacity(name_count);

        let mut i = 0u;
        unsafe {
            while i < name_count {
                let n: uint = (ptr::read_ptr(tabptr as *mut c_uchar) as uint << 8) | (ptr::read_ptr(ptr::offset(tabptr, 1) as *mut c_uchar) as uint);
                let name_cstring = c_str::CString::new(ptr::offset(tabptr, 2) as *c_char, false);
                let name: ~str = name_cstring.as_str().unwrap().to_owned();
                let n_vec = name_table.find_or_insert_with(name, |_| -> ~[uint] {
                    ~[]
                });
                n_vec.push(n);
                tabptr = ptr::offset(tabptr, name_entry_size as int);
                i += 1;
            }
        }

        name_table
    }

    pub fn study(&mut self) -> bool {
        self.study_with_options(0)
    }

    pub fn study_with_options(&mut self, options: study_options) -> bool {
        // Free any current study data.
        detail::pcre_free_study(self.extra);
        self.extra = ptr::null();

        let extra = detail::pcre_study(self.code, options);
        self.extra = extra;
        ptr::is_not_null(extra)
    }
}

impl Drop for Pcre {
    fn drop(&mut self) {
        detail::pcre_free_study(self.extra);
        self.extra = ptr::null();
        detail::pcre_free(self.code as *c_void);
        self.code = ptr::null();
    }
}

impl<'self> Match<'self> {

    pub fn group_start(&self, n: uint) -> uint {
        self.ovector[(n * 2) as uint] as uint
    }

    pub fn group_end(&self, n: uint) -> uint {
        self.ovector[(n * 2 + 1) as uint] as uint
    }

    pub fn group_len(&self, n: uint) -> uint {
        let group_offsets = self.ovector.slice_from((n * 2) as uint);
        (group_offsets[1] - group_offsets[0]) as uint
    }

    pub fn group(&self, n: uint) -> &'self str {
        let group_offsets = self.ovector.slice_from((n * 2) as uint);
        let start = group_offsets[0];
        let end = group_offsets[1];
        self.subject.slice(start as uint, end as uint)
    }

    pub fn string_count(&self) -> uint {
        self.string_count_
    }
}

pub fn pcre_version() -> ~str {
    detail::pcre_version()
}

#[cfg(test)]
mod test {

    #[test]
    #[should_fail]
    fn test_compile_nul() {
        // Nul bytes are not allowed in the pattern string.
        ::Pcre::compile("\0abc");
    }

    #[test]
    #[should_fail]
    fn test_compile_bad_pattern() {
        ::Pcre::compile("[");
    }

    #[test]
    fn test_compile_capture_count() {
        let re = ::Pcre::compile("(?:abc)(def)");
        assert_eq!(re.capture_count(), 1u);
    }

    #[test]
    fn test_exec_basic() {
        let re = ::Pcre::compile("^...$");
        assert_eq!(re.capture_count(), 0u);
        let m = re.exec("abc").unwrap();
        assert_eq!(m.group(0), "abc");
    }

    #[test]
    fn test_exec_no_match() {
        let re = ::Pcre::compile("abc");
        assert!(re.exec("def").is_none());
    }

    #[test]
    fn test_exec_nul_byte() {
        // Nul bytes *are* allowed in subject strings, however.
        let re = ::Pcre::compile("abc\\0def");
        let m = re.exec("abc\0def").unwrap();
        assert_eq!(m.group(0), "abc\0def");
    }

    #[test]
    fn test_exec_from_basic() {
        let re = ::Pcre::compile("abc");
        let subject = "abcabc";
        let m1 = re.exec_from(subject, 1u).unwrap();
        assert_eq!(m1.group_start(0u), 3u);
        assert_eq!(m1.group_end(0u), 6u);
        assert_eq!(m1.group_len(0u), 3u);
        let m2 = re.exec(subject).unwrap();
        assert_eq!(m2.group_start(0u), 0u);
    }

    #[test]
    fn test_study_basic() {
        let mut re = ::Pcre::compile("abc");
        let mut study_res = re.study();
        assert!(study_res);
        // Re-study the pattern two more times (to check for leaks when the test program
        // is run through Valgrind).
        study_res = re.study();
        assert!(study_res);
        study_res = re.study();
        assert!(study_res);
    }
}
