// Copyright 2013 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[feature(link_args)];

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

#[deriving(Clone)]
pub enum CompileOption {
    Caseless = 0x00000001,
    Multiline = 0x00000002,
    DotAll = 0x00000004,
    Extended = 0x00000008,
    Anchored = 0x00000010,
    DollarEndOnly = 0x00000020,
    Extra = 0x00000040,
    Ungreedy = 0x00000200,
    NoAutoCapture = 0x00001000,
    AutoCallout = 0x00004000,
    FirstLine = 0x00040000,
    DupNames = 0x00080000,
    NewlineCR = 0x00100000,
    NewlineLF = 0x00200000,
    NewlineCRLF = 0x00300000,
    NewlineAny = 0x00400000,
    NewlineAnyCRLF = 0x00500000,
    BsrAnyCRLF = 0x00800000,
    BsrUnicode = 0x01000000,
    JavaScriptCompat = 0x02000000,
    Ucp = 0x20000000
}

#[deriving(Clone)]
pub enum StudyOption {
    StudyJitCompile = 0x0001,
    StudyJitPartialSoftCompile = 0x0002,
    StudyJitPartialHardCompile = 0x0004,
    StudyExtraNeeded = 0x0008
}

#[deriving(Clone)]
pub enum ExecOption {
    ExecAnchored = 0x00000010,
    ExecNotBol = 0x00000080,
    ExecNotEol = 0x00000100,
    ExecNotEmpty = 0x00000400,
    ExecPartialSoft = 0x00008000,
    ExecNewlineCR = 0x00100000,
    ExecNewlineLF = 0x00200000,
    ExecNewlineCRLF = 0x00300000,
    ExecNewlineAny = 0x00400000,
    ExecNewlineAnyCRLF = 0x00500000,
    ExecBsrAnyCRLF = 0x00800000,
    ExecBsrUnicode = 0x01000000,
    ExecNoStartOptimise = 0x04000000,
    ExecPartialHard = 0x08000000,
    ExecNotEmptyAtStart = 0x10000000
}

pub static ExecPartial: ExecOption = ExecPartialSoft;
pub static ExecNoStartOptimize: ExecOption = ExecNoStartOptimise;

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
pub struct Match<'a> {

    priv subject: &'a str,

    priv partial_ovector: ~[c_int],

    priv string_count_: c_int

}

/// Iterator type for iterating matches within a subject string.
pub struct MatchIterator<'a> {

    priv code: *detail::pcre,

    priv extra: *detail::pcre_extra,

    priv capture_count: c_int,

    priv subject: &'a str,

    /// The subject string as a `CString`. In MatchIterator's next() method, this is re-used
    /// each time so that only one C-string copy of the subject string needs to be allocated.
    priv subject_cstring: c_str::CString,

    priv offset: c_int,

    priv options: EnumSet<ExecOption>,

    priv ovector: ~[c_int]

}

impl CLike for CompileOption {
    fn from_uint(n: uint) -> CompileOption {
        match n {
            1u => Caseless,
            2u => Multiline,
            3u => DotAll,
            4u => Extended,
            5u => Anchored,
            6u => DollarEndOnly,
            7u => Extra,
            8u => Ungreedy,
            9u => NoAutoCapture,
            10u => AutoCallout,
            11u => FirstLine,
            12u => DupNames,
            13u => NewlineCR,
            14u => NewlineLF,
            15u => NewlineCRLF,
            16u => NewlineAny,
            17u => NewlineAnyCRLF,
            18u => BsrAnyCRLF,
            19u => BsrUnicode,
            20u => JavaScriptCompat,
            21u => Ucp,
            _ => fail!("unknown CompileOption number {:u}", n)
        }
    }

    fn to_uint(&self) -> uint {
        match *self {
            Caseless => 1u,
            Multiline => 2u,
            DotAll => 3u,
            Extended => 4u,
            Anchored => 5u,
            DollarEndOnly => 6u,
            Extra => 7u,
            Ungreedy => 8u,
            NoAutoCapture => 9u,
            AutoCallout => 10u,
            FirstLine => 11u,
            DupNames => 12u,
            NewlineCR => 13u,
            NewlineLF => 14u,
            NewlineCRLF => 15u,
            NewlineAny => 16u,
            NewlineAnyCRLF => 17u,
            BsrAnyCRLF => 18u,
            BsrUnicode => 19u,
            JavaScriptCompat => 20u,
            Ucp => 21u
        }
    }
}

impl CLike for StudyOption {
    fn from_uint(n: uint) -> StudyOption {
        match n {
            1u => StudyJitCompile,
            2u => StudyJitPartialSoftCompile,
            3u => StudyJitPartialHardCompile,
            4u => StudyExtraNeeded,
            _ => fail!("unknown StudyOption number {:u}", n)
        }
    }

    fn to_uint(&self) -> uint {
        match *self {
            StudyJitCompile => 1u,
            StudyJitPartialSoftCompile => 2u,
            StudyJitPartialHardCompile => 3u,
            StudyExtraNeeded => 4u
        }
    }
}

impl CLike for ExecOption {
    fn from_uint(n: uint) -> ExecOption {
        match n {
            1u => ExecAnchored,
            2u => ExecNotBol,
            3u => ExecNotEol,
            4u => ExecNotEmpty,
            5u => ExecPartialSoft,
            6u => ExecNewlineCR,
            7u => ExecNewlineLF,
            8u => ExecNewlineCRLF,
            9u => ExecNewlineAny,
            10u => ExecNewlineAnyCRLF,
            11u => ExecBsrAnyCRLF,
            12u => ExecBsrUnicode,
            13u => ExecNoStartOptimise,
            14u => ExecPartialHard,
            15u => ExecNotEmptyAtStart,
            _ => fail!("unknown ExecOption number {:u}", n)
        }
    }

    fn to_uint(&self) -> uint {
        match *self {
            ExecAnchored => 1u,
            ExecNotBol => 2u,
            ExecNotEol => 3u,
            ExecNotEmpty => 4u,
            ExecPartialSoft => 5u,
            ExecNewlineCR => 6u,
            ExecNewlineLF => 7u,
            ExecNewlineCRLF => 8u,
            ExecNewlineAny => 9u,
            ExecNewlineAnyCRLF => 10u,
            ExecBsrAnyCRLF => 11u,
            ExecBsrUnicode => 12u,
            ExecNoStartOptimise => 13u,
            ExecPartialHard => 14u,
            ExecNotEmptyAtStart => 15u
        }
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
        let no_options: EnumSet<CompileOption> = EnumSet::empty();
        Pcre::compile_with_options(pattern, &no_options)
    }

    /// Compiles a regular expression using the given bitwise-OR'd options `options`.
    ///
    /// # Arguments
    /// * `pattern` - The regular expression.
    /// * `options` - Bitwise-OR'd compilation options. See the libpcre manpages,
    ///   `man 3 pcre_compile`, for more information.
    pub fn compile_with_options(pattern: &str, options: &EnumSet<CompileOption>) -> Result<Pcre, CompilationError> {
        pattern.with_c_str(|pattern_c_str| {
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
        })
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
        let no_options: EnumSet<ExecOption> = EnumSet::empty();
        self.exec_from_with_options(subject, startoffset, &no_options)
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
    pub fn exec_from_with_options<'a>(&self, subject: &'a str, startoffset: uint, options: &EnumSet<ExecOption>) -> Option<Match<'a>> {
        let ovecsize = (self.capture_count_ + 1) * 3;
        let mut ovector: ~[c_int] = vec::from_elem(ovecsize as uint, 0 as c_int);

        unsafe {
            subject.with_c_str_unchecked(|subject_c_str| -> Option<Match<'a>> {
                let rc = detail::pcre_exec(self.code, self.extra, subject_c_str, subject.len() as c_int, startoffset as c_int, options, ovector.as_mut_ptr(), ovecsize as c_int);
                if rc >= 0 {
                    Some(Match {
                        subject: subject,
                        partial_ovector: ovector.slice_to(((self.capture_count_ + 1) * 2) as uint).to_owned(),
                        string_count_: rc
                    })
                } else {
                    None
                }
            })
        }
    }

    /// Creates a `MatchIterator` for iterating through matches within the given subject
    /// string `subject`.
    ///
    /// # Argument
    /// * `subject` - The subject string.
    #[inline]
    pub fn matches<'a>(&self, subject: &'a str) -> MatchIterator<'a> {
        let no_options: EnumSet<ExecOption> = EnumSet::empty();
        self.matches_with_options(subject, &no_options)
    }

    /// Creates a `MatchIterator` for iterating through matches within the given subject
    /// string `subject` using the given bitwise-OR'd matching options `options`.
    ///
    /// # Arguments
    /// * `subject` - The subject string.
    /// * `options` - Bitwise-OR'd matching options. See the libpcre manpages, `man 3 pcre_exec`,
    ///   for more information.
    #[inline]
    pub fn matches_with_options<'a>(&self, subject: &'a str, options: &EnumSet<ExecOption>) -> MatchIterator<'a> {
        unsafe {
            let ovecsize = (self.capture_count_ + 1) * 3;
            MatchIterator {
                code: { detail::pcre_refcount(self.code as *mut detail::pcre, 1); self.code },
                extra: self.extra,
                capture_count: self.capture_count_,
                subject: subject,
                subject_cstring: subject.to_c_str_unchecked(), // the subject string can contain NUL bytes
                offset: 0,
                options: options.clone(),
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

impl<'a> Match<'a> {
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
    pub fn group(&self, n: uint) -> &'a str {
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

impl<'a> Clone for MatchIterator<'a> {
    #[inline]
    fn clone(&self) -> MatchIterator<'a> {
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
impl<'a> Drop for MatchIterator<'a> {
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

impl<'a> Iterator<Match<'a>> for MatchIterator<'a> {
    /// Gets the next match.
    #[inline]
    fn next(&mut self) -> Option<Match<'a>> {
        unsafe {
            self.subject_cstring.with_ref(|subject_c_str| -> Option<Match<'a>> {
                let rc = detail::pcre_exec(self.code, self.extra, subject_c_str, self.subject.len() as c_int, self.offset, &self.options, self.ovector.as_mut_ptr(), self.ovector.len() as c_int);
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
            })
        }
    }
}

/// Returns libpcre version information.
pub fn pcre_version() -> ~str {
    detail::pcre_version()
}
