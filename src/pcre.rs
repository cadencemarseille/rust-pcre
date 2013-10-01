// Copyright 2013 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[link(name = "pcre", vers = "0.1")];
#[crate_type = "lib"];

use std::libc::{c_int, c_uchar, c_void};
use std::option::{Option};
use std::ptr;
use std::vec;

pub type options = c_int;

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

mod detail {

    use std::c_str::*;
    use std::libc::*;
    use std::ptr;

    pub type fullinfo_field = c_int;
    pub type pcre = c_void;
    pub type pcre_error = c_int;
    pub type pcre_extra = c_void;

    pub static PCRE_ERROR_NOMATCH: pcre_error = -1;
    pub static PCRE_ERROR_NULL: pcre_error = -2;

    pub static PCRE_INFO_CAPTURECOUNT: fullinfo_field = 2;

    mod native {

        use std::libc::*;

        #[link_args = "-lpcre.1"]
        extern {
            static pcre_free: extern "C" fn(ptr: *c_void);

            fn pcre_compile(pattern: *c_char, options: ::options, errptr: *mut *c_char, erroffset: *mut c_int, tableptr: *c_uchar) -> *::detail::pcre;
            fn pcre_exec(code: *::detail::pcre, extra: *::detail::pcre_extra, subject: *c_char, length: c_int, startoffset: c_int, options: ::options, ovector: *mut c_int, ovecsize: c_int) -> ::detail::pcre_error;
            fn pcre_free_study(extra: *::detail::pcre_extra);
            fn pcre_fullinfo(code: *::detail::pcre, extra: *::detail::pcre_extra, what: ::detail::fullinfo_field, where: *mut c_void) -> c_int;
        }

    }

    #[fixed_stack_segment]
    #[inline(never)]
    pub fn pcre_compile(pattern: *c_char, options: ::options, tableptr: *c_uchar) -> *pcre {
        assert!(ptr::is_not_null(pattern));
        let mut err: *c_char = ptr::null();
        let mut erroffset: c_int = 0;
        let code = unsafe { native::pcre_compile(pattern, options, &mut err, &mut erroffset, tableptr) };

        if ptr::is_null(code) {
            // "Otherwise, if  compilation  of  a  pattern fails, pcre_compile() returns
            // NULL, and sets the variable pointed to by errptr to point to a textual
            // error message. This is a static string that is part of the library. You
            // must not try to free it."
            // http://pcre.org/pcre.txt
            let err_cstring = unsafe { CString::new(err, false) };
            match err_cstring.as_str() {
                None          => error!("pcre_compile() failed at offset %u", erroffset as uint),
                Some(err_str) => error!("pcre_compile() failed at offset %u: %s", erroffset as uint, err_str)
            }
            fail!("pcre_compile");
        }
        assert!(ptr::is_not_null(code) && erroffset == 0);

        code
    }

    #[fixed_stack_segment]
    #[inline(never)]
    pub fn pcre_exec(code: *pcre, extra: *pcre_extra, subject: *c_char, length: c_int, startoffset: c_int, options: ::options, ovector: *mut c_int, ovecsize: c_int) -> bool {
        assert!(ptr::is_not_null(code));
        assert!(ovecsize >= 0 && ovecsize % 3 == 0);
        let rc = unsafe { native::pcre_exec(code, extra, subject, length, startoffset, options, ovector, ovecsize) };
        if rc == PCRE_ERROR_NOMATCH {
            return false;
        } else if rc < 0 && rc != PCRE_ERROR_NULL {
            fail!("pcre_exec");
        }

        true
    }

    #[fixed_stack_segment]
    #[inline(never)]
    pub fn pcre_free(ptr: *c_void) {
        native::pcre_free(ptr);
    }

    #[fixed_stack_segment]
    #[inline(never)]
    pub fn pcre_free_study(extra: *pcre_extra) {
        unsafe { native::pcre_free_study(extra); }
    }

    #[fixed_stack_segment]
    #[inline(never)]
    pub fn pcre_fullinfo(code: *pcre, extra: *pcre_extra, what: fullinfo_field, where: *mut c_void) {
        assert!(ptr::is_not_null(code));
        let rc = unsafe { native::pcre_fullinfo(code, extra, what, where) };
        if rc < 0 && rc != PCRE_ERROR_NULL {
            fail!("pcre_fullinfo");
        }
    }

} // End `mod detail`.

pub struct Pcre {

    priv code: *detail::pcre,

    priv extra: *detail::pcre_extra,

    capture_count: uint

}

pub struct Match<'self> {

    priv subject: &'self str,

    priv ovector: ~[c_int]

}

impl Pcre {
    fn compile(pattern: &str) -> Pcre {
        Pcre::compile_with_options(pattern, 0)
    }

    fn compile_with_options(pattern: &str, options: options) -> Pcre {
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
                capture_count: capture_count as uint
            }
        }
    }

    fn exec<'a>(&self, subject: &'a str) -> Option<Match<'a>> {
        self.exec_from(subject, 0)
    }

    fn exec_from<'a>(&self, subject: &'a str, startoffset: uint) -> Option<Match<'a>> {
        self.exec_from_with_options(subject, startoffset, 0)
    }

    fn exec_from_with_options<'a>(&self, subject: &'a str, startoffset: uint, options: options) -> Option<Match<'a>> {
        let ovecsize = (self.capture_count + 1) * 3;
        let mut ovector: ~[c_int] = vec::from_elem(ovecsize, 0 as c_int);

        unsafe {
            do subject.with_c_str_unchecked |subject_c_str| -> Option<Match<'a>> {
                if detail::pcre_exec(self.code, self.extra, subject_c_str, subject.len() as c_int, startoffset as c_int, options, vec::raw::to_mut_ptr(ovector), ovecsize as c_int) {
                    Some(Match {
                        subject: subject,
                        // TODO: Is it possible to avoid to_owned()?
                        // Probably need multiple lifetime parameters:
                        // https://mail.mozilla.org/pipermail/rust-dev/2013-September/005829.html
                        ovector: ovector.slice_to((self.capture_count + 1) * 2).to_owned()
                    })
                } else {
                    None
                }
            }
        }
    }
}

impl Drop for Pcre {
    fn drop(&mut self) {
        detail::pcre_free_study(self.extra);
        self.extra = ptr::null();
        detail::pcre_free(self.code);
        self.code = ptr::null();
    }
}

impl<'self> Match<'self> {

    fn group_start(&self, n: uint) -> uint {
        self.ovector[(n * 2) as uint] as uint
    }

    fn group_end(&self, n: uint) -> uint {
        self.ovector[(n * 2 + 1) as uint] as uint
    }

    fn group_len(&self, n: uint) -> uint {
        let group_offsets = self.ovector.slice_from((n * 2) as uint);
        (group_offsets[1] - group_offsets[0]) as uint
    }

    fn group(&self, n: uint) -> &'self str {
        let group_offsets = self.ovector.slice_from((n * 2) as uint);
        let start = group_offsets[0];
        let end = group_offsets[1];
        self.subject.slice(start as uint, end as uint)
    }

}

#[cfg(test)]
mod tests {

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
        assert_eq!(re.capture_count, 1u);
    }

    #[test]
    fn test_exec_basic() {
        let re = ::Pcre::compile("^...$");
        assert_eq!(re.capture_count, 0u);
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
}
