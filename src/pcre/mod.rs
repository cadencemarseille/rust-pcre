// Copyright 2013 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[link(name = "pcre", package_id = "pcre", vers = "0.1")];
#[crate_type = "lib"];

extern mod extra;

use extra::enum_set::{CLike, EnumSet};
use extra::treemap::{TreeMap};
use std::c_str;
use std::libc::{c_char, c_int, c_uchar, c_void};
use std::option::{Option};
use std::ptr;
use std::result::{Result};
use std::vec;

mod detail;

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

pub enum StudyOption {
    JitCompile = 0x0001,
    JitPartialSoftCompile = 0x0002,
    JitPartialHardCompile = 0x0004,
    ExtraNeeded = 0x0008
}

pub struct CompilationError {

    priv opt_err: Option<~str>,

    priv erroffset: c_int

}

/// Wrapper for libpcre's `pcre` object (representing a compiled regular expression).
pub struct Pcre {

    priv code: *detail::pcre,

    priv extra: *detail::pcre_extra,

    priv capture_count_: c_int

}

/// Represents a match of a subject string against a regular expression.
pub struct Match<'self> {

    priv subject: &'self str,

    priv partial_ovector: ~[c_int],

    priv string_count_: c_int

}

/// Iterator type for iterating matches within a subject string.
pub struct MatchIterator<'self> {

    priv code: *detail::pcre,

    priv extra: *detail::pcre_extra,

    priv capture_count: c_int,

    priv subject: &'self str,

    /// The subject string as a `CString`. In MatchIterator's next() method, this is re-used
    /// each time so that only one C-string copy of the subject string needs to be allocated.
    priv subject_cstring: c_str::CString,

    priv offset: c_int,

    priv options: options,

    priv ovector: ~[c_int]

}

impl CLike for StudyOption {
    fn from_uint(v: uint) -> StudyOption {
        match v {
            0x0001 => JitCompile,
            0x0002 => JitPartialSoftCompile,
            0x0004 => JitPartialHardCompile,
            0x0008 => ExtraNeeded,
            _ => fail!("unknown StudyOption value {:u}", v)
        }
    }

    fn to_uint(&self) -> uint {
        *self as uint
    }
}

impl CompilationError {
    pub fn message(&self) -> Option<~str> {
        self.opt_err.clone()
    }

    pub fn offset(&self) -> uint {
        self.erroffset as uint
    }
}

impl ToStr for CompilationError {
    fn to_str(&self) -> ~str {
        match self.opt_err {
            None => format!("compilation failed at offset {:u}", self.erroffset as uint),
            Some(ref s) => format!("compilation failed at offset {:u}: {:s}", self.erroffset as uint, s.as_slice())
        }
    }
}

impl Pcre {
    /// Compiles the given regular expression.
    ///
    /// # Argument
    /// * `pattern` - The regular expression.
    pub fn compile(pattern: &str) -> Result<Pcre, CompilationError> {
        Pcre::compile_with_options(pattern, 0)
    }

    /// Compiles a regular expression using the given bitwise-OR'd options `options`.
    ///
    /// # Arguments
    /// * `pattern` - The regular expression.
    /// * `options` - Bitwise-OR'd compilation options. See the libpcre manpages,
    ///   `man 3 pcre_compile`, for more information.
    pub fn compile_with_options(pattern: &str, options: options) -> Result<Pcre, CompilationError> {
        do pattern.with_c_str |pattern_c_str| {
            unsafe {
                // Use the default character tables.
                let tableptr: *c_uchar = ptr::null();
                match detail::pcre_compile(pattern_c_str, options, tableptr) {
                    Err((opt_err, erroffset)) => Err(CompilationError {
                        opt_err: opt_err,
                        erroffset: erroffset
                    }),
                    Ok(mut_code) => {
                        let code = mut_code as *detail::pcre;
                        assert!(ptr::is_not_null(code));
                        // Take a reference.
                        detail::pcre_refcount(code as *mut detail::pcre, 1);

                        let extra: *detail::pcre_extra = ptr::null();

                        let mut capture_count: c_int = 0;
                        detail::pcre_fullinfo(code, extra, detail::PCRE_INFO_CAPTURECOUNT, &mut capture_count as *mut c_int as *mut c_void);

                        Ok(Pcre {
                            code: code,
                            extra: extra,
                            capture_count_: capture_count
                        })
                    }
                }
            }
        }
    }

    /// Returns the number of capture groups in the regular expression, including one for
    /// each named capture group.
    ///
    /// This count does not include "group 0", which is the full substring within a subject
    /// string that matches the regular expression.
    ///
    /// # See also
    /// * [name_count()](#fn.name_count) - Returns the number of named capture groups.
    pub fn capture_count(&self) -> uint {
        self.capture_count_ as uint
    }

    /// Matches the compiled regular expression against a given subject string `subject`.
    /// If no match is found, then `None` is returned. Otherwise, a `Match` object is returned
    /// which provides access to the captured substrings as slices of the subject string.
    ///
    /// # Argument
    /// * `subject` - The subject string.
    ///
    /// # Performance notes
    /// This method is intended to be used to find individual matches. If multiple matches
    /// are desired, then a `MatchIterator` should be used because it is more efficient.
    ///
    /// If a regular expression will be used often, it might be worth studying it to possibly
    /// speed up matching. See the [study()](#fn.study) method.
    #[inline]
    pub fn exec<'a>(&self, subject: &'a str) -> Option<Match<'a>> {
        self.exec_from(subject, 0)
    }

    /// Matches the compiled regular expression against a given subject string `subject`
    /// starting at offset `startoffset` within the subject string. If no match is found,
    /// then `None` is returned. Otherwise, a `Match` object is returned which provides
    /// access to the captured substrings as slices of the subject string.
    ///
    /// # Arguments
    /// * `subject` - The subject string.
    /// * `startoffset` - Starting offset within `subject` at which to begin looking for
    ///   a match.
    ///
    /// # Performance notes
    /// This method is intended to be used to find individual matches. If multiple matches
    /// are desired, then a `MatchIterator` should be used because it is more efficient.
    ///
    /// If a regular expression will be used often, it might be worth studying it to possibly
    /// speed up matching. See the [study()](#fn.study) method.
    #[inline]
    pub fn exec_from<'a>(&self, subject: &'a str, startoffset: uint) -> Option<Match<'a>> {
        self.exec_from_with_options(subject, startoffset, 0)
    }

    /// Matches the compiled regular expression against a given subject string `subject`
    /// starting at offset `startoffset` within the subject string and using the given
    /// bitwise-OR'd matching options `options`. If no match is found, then `None` is
    /// returned. Otherwise, a `Match` object is returned which provides access to the
    /// captured substrings as slices of the subject string.
    ///
    /// # Arguments
    /// * `subject` - The subject string.
    /// * `startoffset` - Starting offset within `subject` at which to begin looking for
    ///   a match.
    /// * `options` - Bitwise-OR'd matching options. See the libpcre manpages, `man 3 pcre_exec`,
    ///   for more information.
    ///
    /// # Performance notes
    /// This method is intended to be used to find individual matches. If multiple matches
    /// are desired, then a `MatchIterator` should be used because it is more efficient.
    ///
    /// If a regular expression will be used often, it might be worth studying it to possibly
    /// speed up matching. See the [study()](#fn.study) method.
    #[inline]
    pub fn exec_from_with_options<'a>(&self, subject: &'a str, startoffset: uint, options: options) -> Option<Match<'a>> {
        let ovecsize = (self.capture_count_ + 1) * 3;
        let mut ovector: ~[c_int] = vec::from_elem(ovecsize as uint, 0 as c_int);

        unsafe {
            do subject.with_c_str_unchecked |subject_c_str| -> Option<Match<'a>> {
                let rc = detail::pcre_exec(self.code, self.extra, subject_c_str, subject.len() as c_int, startoffset as c_int, options, vec::raw::to_mut_ptr(ovector), ovecsize as c_int);
                if rc >= 0 {
                    Some(Match {
                        subject: subject,
                        partial_ovector: ovector.slice_to(((self.capture_count_ + 1) * 2) as uint).to_owned(),
                        string_count_: rc
                    })
                } else {
                    None
                }
            }
        }
    }

    /// Creates a `MatchIterator` for iterating through matches within the given subject
    /// string `subject`.
    ///
    /// # Argument
    /// * `subject` - The subject string.
    #[inline]
    pub fn match_iter<'a>(&self, subject: &'a str) -> MatchIterator<'a> {
        self.match_iter_with_options(subject, 0)
    }

    /// Creates a `MatchIterator` for iterating through matches within the given subject
    /// string `subject` using the given bitwise-OR'd matching options `options`.
    ///
    /// # Arguments
    /// * `subject` - The subject string.
    /// * `options` - Bitwise-OR'd matching options. See the libpcre manpages, `man 3 pcre_exec`,
    ///   for more information.
    #[inline]
    pub fn match_iter_with_options<'a>(&self, subject: &'a str, options: options) -> MatchIterator<'a> {
        unsafe {
            let ovecsize = (self.capture_count_ + 1) * 3;
            MatchIterator {
                code: { detail::pcre_refcount(self.code as *mut detail::pcre, 1); self.code },
                extra: self.extra,
                capture_count: self.capture_count_,
                subject: subject,
                subject_cstring: subject.to_c_str_unchecked(),
                offset: 0,
                options: options,
                ovector: vec::from_elem(ovecsize as uint, 0 as c_int)
            }
        }
    }

    /// Returns the number of named capture groups in the regular expression.
    pub fn name_count(&self) -> uint {
        unsafe {
            let mut name_count: c_int = 0;
            detail::pcre_fullinfo(self.code, self.extra, detail::PCRE_INFO_NAMECOUNT, &mut name_count as *mut c_int as *mut c_void);
            name_count as uint
        }
    }

    /// Creates a name-to-number translation table that maps the name of each named capture
    /// group to the assigned group numbers.
    ///
    /// The value type of the returned `TreeMap` is a `uint` vector because there can be
    /// more than one group number for a given name if the PCRE_DUPNAMES option is used
    /// when compiling the regular expression.
    pub fn name_table(&self) -> TreeMap<~str, ~[uint]> {
        unsafe {
            let name_count = self.name_count();
            let mut tabptr: *c_uchar = ptr::null();
            detail::pcre_fullinfo(self.code, self.extra, detail::PCRE_INFO_NAMETABLE, &mut tabptr as *mut *c_uchar as *mut c_void);
            let mut name_entry_size: c_int = 0;
            detail::pcre_fullinfo(self.code, self.extra, detail::PCRE_INFO_NAMEENTRYSIZE, &mut name_entry_size as *mut c_int as *mut c_void);

            let mut name_table: TreeMap<~str, ~[uint]> = TreeMap::new();

            let mut i = 0u;
            while i < name_count {
                let n: uint = (ptr::read_ptr(tabptr) as uint << 8) | (ptr::read_ptr(ptr::offset(tabptr, 1)) as uint);
                let name_cstring = c_str::CString::new(ptr::offset(tabptr, 2) as *c_char, false);
                let name: ~str = name_cstring.as_str().unwrap().to_owned();
                // TODO Avoid the double lookup.
                // https://github.com/mozilla/rust/issues/9068
                if !name_table.contains_key(&name) {
                    name_table.insert(name, ~[n]);
                } else {
                    name_table.find_mut(&name).unwrap().push(n);
                }
                tabptr = ptr::offset(tabptr, name_entry_size as int);
                i += 1;
            }

            name_table
        }
    }

    /// Studies the regular expression to see if additional information can be extracted
    /// which might speed up matching.
    ///
    /// # Return value
    /// `true` if additional information could be extracted. `false` otherwise.
    pub fn study(&mut self) -> bool {
        let no_options: EnumSet<StudyOption> = EnumSet::empty();
        self.study_with_options(&no_options)
    }

    /// Studies the regular expression using the given bitwise-OR'd study options `options`
    /// to see if additional information can be extracted which might speed up matching.
    ///
    /// # Argument
    /// * `options` - Study options. See the libpcre manpages, `man 3 pcre_study`, for more
    ///   information about each option.
    ///
    /// # Return value
    /// `true` if additional information could be extracted. `false` otherwise.
    pub fn study_with_options(&mut self, options: &EnumSet<StudyOption>) -> bool {
        unsafe {
            // If something else has a reference to `code` then it probably has a pointer to
            // the current study data (if any). Thus, we shouldn't free the current study data
            // in that case.
            if detail::pcre_refcount(self.code as *mut detail::pcre, 0) != 1 {
                false
            } else {
                // Free any current study data.
                detail::pcre_free_study(self.extra as *mut detail::pcre_extra);
                self.extra = ptr::null();

                let extra = detail::pcre_study(self.code, options) as *detail::pcre_extra;
                self.extra = extra;
                ptr::is_not_null(extra)
            }
        }
    }
}

impl Drop for Pcre {
    fn drop(&mut self) {
        unsafe {
            if detail::pcre_refcount(self.code as *mut detail::pcre, -1) == 0 {
                detail::pcre_free_study(self.extra as *mut detail::pcre_extra);
                detail::pcre_free(self.code as *mut detail::pcre as *mut c_void);
            }
            self.extra = ptr::null();
            self.code = ptr::null();
        }
    }
}

impl<'self> Match<'self> {
    /// Returns the start index within the subject string of capture group `n`.
    pub fn group_start(&self, n: uint) -> uint {
        self.partial_ovector[(n * 2) as uint] as uint
    }

    /// Returns the end index within the subject string of capture group `n`.
    pub fn group_end(&self, n: uint) -> uint {
        self.partial_ovector[(n * 2 + 1) as uint] as uint
    }

    /// Returns the length of the substring for capture group `n`.
    pub fn group_len(&self, n: uint) -> uint {
        let group_offsets = self.partial_ovector.slice_from((n * 2) as uint);
        (group_offsets[1] - group_offsets[0]) as uint
    }

    /// Returns the substring for capture group `n` as a slice.
    #[inline]
    pub fn group(&self, n: uint) -> &'self str {
        let group_offsets = self.partial_ovector.slice_from((n * 2) as uint);
        let start = group_offsets[0];
        let end = group_offsets[1];
        self.subject.slice(start as uint, end as uint)
    }

    /// Returns the number of substrings captured.
    pub fn string_count(&self) -> uint {
        self.string_count_ as uint
    }
}

impl<'self> Clone for MatchIterator<'self> {
    #[inline]
    fn clone(&self) -> MatchIterator<'self> {
        unsafe {
            MatchIterator {
                code: { detail::pcre_refcount(self.code as *mut detail::pcre, 1); self.code },
                extra: self.extra,
                capture_count: self.capture_count,
                subject: self.subject,
                subject_cstring: self.subject.to_c_str_unchecked(),
                offset: self.offset,
                options: self.options,
                ovector: self.ovector.clone()
            }
        }
    }
}

#[unsafe_destructor]
impl<'self> Drop for MatchIterator<'self> {
    fn drop(&mut self) {
        unsafe {
            if detail::pcre_refcount(self.code as *mut detail::pcre, -1) == 0 {
                detail::pcre_free_study(self.extra as *mut detail::pcre_extra);
                detail::pcre_free(self.code as *mut detail::pcre as *mut c_void);
            }
            self.extra = ptr::null();
            self.code = ptr::null();
        }
    }
}

impl<'self> Iterator<Match<'self>> for MatchIterator<'self> {
    /// Gets the next match.
    #[inline]
    fn next(&mut self) -> Option<Match<'self>> {
        unsafe {
            do self.subject_cstring.with_ref |subject_c_str| -> Option<Match<'self>> {
                let rc = detail::pcre_exec(self.code, self.extra, subject_c_str, self.subject.len() as c_int, self.offset, self.options, vec::raw::to_mut_ptr(self.ovector), self.ovector.len() as c_int);
                if rc >= 0 {
                    // Update the iterator state.
                    self.offset = self.ovector[1];

                    Some(Match {
                        subject: self.subject,
                        partial_ovector: self.ovector.slice_to(((self.capture_count + 1) * 2) as uint).to_owned(),
                        string_count_: rc
                    })
                } else {
                    None
                }
            }
        }
    }
}

/// Returns libpcre version information.
pub fn pcre_version() -> ~str {
    detail::pcre_version()
}
