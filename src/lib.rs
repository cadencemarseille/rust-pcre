// Copyright 2015 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate enum_set;
extern crate libc;
extern crate libpcre_sys;

use enum_set::{CLike, EnumSet};
use libc::{c_char, c_int, c_uchar, c_void};
use std::collections::{BTreeMap};
use std::ffi::{CStr, CString};
use std::marker::{PhantomData};
use std::option::{Option};
use std::ptr;
use std::result::{Result};
use std::string::{String};
use std::vec::{Vec};

mod detail;

#[derive(Clone)]
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

#[derive(Clone)]
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

#[allow(non_upper_case_globals)]
pub const ExecPartial: ExecOption = ExecOption::ExecPartialSoft;
#[allow(non_upper_case_globals)]
pub const ExecNoStartOptimize: ExecOption = ExecOption::ExecNoStartOptimise;

#[derive(Clone)]
pub enum StudyOption {
    StudyJitCompile = 0x0001,
    StudyJitPartialSoftCompile = 0x0002,
    StudyJitPartialHardCompile = 0x0004,

    /// Always create an extra block. Note: Requires PCRE version 8.32 or later.
    StudyExtraNeeded = 0x0008
}

#[derive(Debug)]
pub struct CompilationError {

    opt_err: Option<String>,

    erroffset: c_int

}

/// Wrapper for libpcre's `pcre` object (representing a compiled regular expression).
#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct Pcre {

    code: *const detail::pcre,

    extra: *mut PcreExtra,

    capture_count_: c_int,

    /// A spot to place a pointer-to-mark name string.
    mark_: *mut c_uchar

}

pub type PcreExtra = libpcre_sys::pcre_extra;

/// Represents a match of a subject string against a regular expression.
pub struct Match<'a> {

    subject: &'a str,

    partial_ovector: Vec<c_int>,

    string_count_: c_int

}

/// Iterator type for iterating matches within a subject string.
pub struct MatchIterator<'a, 'p> {

    code: *const detail::pcre,

    extra: *const PcreExtra,

    capture_count: c_int,

    subject: &'a str,

    offset: c_int,

    options: EnumSet<ExecOption>,

    ovector: Vec<c_int>,

    _marker: PhantomData<&'p mut Pcre>

}

impl CLike for CompileOption {
    unsafe fn from_u32(n: u32) -> CompileOption {
        use CompileOption::*;
        match n {
            1 => Caseless,
            2 => Multiline,
            3 => DotAll,
            4 => Extended,
            5 => Anchored,
            6 => DollarEndOnly,
            7 => Extra,
            8 => Ungreedy,
            9 => NoAutoCapture,
            10 => AutoCallout,
            11 => FirstLine,
            12 => DupNames,
            13 => NewlineCR,
            14 => NewlineLF,
            15 => NewlineCRLF,
            16 => NewlineAny,
            17 => NewlineAnyCRLF,
            18 => BsrAnyCRLF,
            19 => BsrUnicode,
            20 => JavaScriptCompat,
            21 => Ucp,
            _ => panic!("unknown CompileOption number {}", n)
        }
    }

    fn to_u32(&self) -> u32 {
        use CompileOption::*;
        match *self {
            Caseless => 1,
            Multiline => 2,
            DotAll => 3,
            Extended => 4,
            Anchored => 5,
            DollarEndOnly => 6,
            Extra => 7,
            Ungreedy => 8,
            NoAutoCapture => 9,
            AutoCallout => 10,
            FirstLine => 11,
            DupNames => 12,
            NewlineCR => 13,
            NewlineLF => 14,
            NewlineCRLF => 15,
            NewlineAny => 16,
            NewlineAnyCRLF => 17,
            BsrAnyCRLF => 18,
            BsrUnicode => 19,
            JavaScriptCompat => 20,
            Ucp => 21
        }
    }
}

impl CLike for ExecOption {
    unsafe fn from_u32(n: u32) -> ExecOption {
        use ExecOption::*;
        match n {
            1 => ExecAnchored,
            2 => ExecNotBol,
            3 => ExecNotEol,
            4 => ExecNotEmpty,
            5 => ExecPartialSoft,
            6 => ExecNewlineCR,
            7 => ExecNewlineLF,
            8 => ExecNewlineCRLF,
            9 => ExecNewlineAny,
            10 => ExecNewlineAnyCRLF,
            11 => ExecBsrAnyCRLF,
            12 => ExecBsrUnicode,
            13 => ExecNoStartOptimise,
            14 => ExecPartialHard,
            15 => ExecNotEmptyAtStart,
            _ => panic!("unknown ExecOption number {}", n)
        }
    }

    fn to_u32(&self) -> u32 {
        use ExecOption::*;
        match *self {
            ExecAnchored => 1,
            ExecNotBol => 2,
            ExecNotEol => 3,
            ExecNotEmpty => 4,
            ExecPartialSoft => 5,
            ExecNewlineCR => 6,
            ExecNewlineLF => 7,
            ExecNewlineCRLF => 8,
            ExecNewlineAny => 9,
            ExecNewlineAnyCRLF => 10,
            ExecBsrAnyCRLF => 11,
            ExecBsrUnicode => 12,
            ExecNoStartOptimise => 13,
            ExecPartialHard => 14,
            ExecNotEmptyAtStart => 15
        }
    }
}

impl CLike for StudyOption {
    unsafe fn from_u32(n: u32) -> StudyOption {
        use StudyOption::*;
        match n {
            1 => StudyJitCompile,
            2 => StudyJitPartialSoftCompile,
            3 => StudyJitPartialHardCompile,
            4 => StudyExtraNeeded,
            _ => panic!("unknown StudyOption number {}", n)
        }
    }

    fn to_u32(&self) -> u32 {
        use StudyOption::*;
        match *self {
            StudyJitCompile => 1,
            StudyJitPartialSoftCompile => 2,
            StudyJitPartialHardCompile => 3,
            StudyExtraNeeded => 4
        }
    }
}

impl CompilationError {
    pub fn message(&self) -> Option<String> {
        self.opt_err.clone()
    }

    pub fn offset(&self) -> usize {
        self.erroffset as usize
    }
}

impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.opt_err {
            None => write!(f, "compilation failed at offset {}", self.erroffset as usize),
            Some(ref s) => write!(f, "compilation failed at offset {}: {}", self.erroffset as usize, s)
        }
    }
}

impl Pcre {
    /// Compiles the given regular expression.
    ///
    /// # Argument
    /// * `pattern` - The regular expression.
    pub fn compile(pattern: &str) -> Result<Pcre, CompilationError> {
        let no_options: EnumSet<CompileOption> = EnumSet::new();
        Pcre::compile_with_options(pattern, &no_options)
    }

    /// Compiles a regular expression using the given bitwise-OR'd options `options`.
    ///
    /// # Arguments
    /// * `pattern` - The regular expression.
    /// * `options` - Bitwise-OR'd compilation options. See the libpcre manpages,
    ///   `man 3 pcre_compile`, for more information.
    pub fn compile_with_options(pattern: &str, options: &EnumSet<CompileOption>) -> Result<Pcre, CompilationError> {
        let pattern_cstring = CString::new(pattern).unwrap();
        unsafe {
            // Use the default character tables.
            let tableptr: *const c_uchar = ptr::null();
            match detail::pcre_compile(pattern_cstring.as_ptr(), options, tableptr) {
                Err((opt_err, erroffset)) => Err(CompilationError {
                    opt_err: opt_err,
                    erroffset: erroffset
                }),
                Ok(mut_code) => {
                    let code = mut_code as *const detail::pcre;
                    assert!(!code.is_null());
                    // Take a reference.
                    detail::pcre_refcount(code as *mut detail::pcre, 1);

                    let extra: *mut PcreExtra = ptr::null_mut();

                    let mut capture_count: c_int = 0;
                    detail::pcre_fullinfo(code, extra as *const PcreExtra, libpcre_sys::PCRE_INFO_CAPTURECOUNT,
                        &mut capture_count as *mut c_int as *mut c_void);

                    Ok(Pcre {
                        code: code,
                        extra: extra,
                        capture_count_: capture_count,
                        mark_: ptr::null_mut()
                    })
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
    /// * [name_count()](#method.name_count) - Returns the number of named capture groups.
    pub fn capture_count(&self) -> usize {
        self.capture_count_ as usize
    }

    /// Enables the use of the mark field when matching the compiled regular expression. The
    /// pattern must have been previously studied and an extra block must have been created.
    ///
    /// To ensure that an extra block has been created, call [study_with_options()](#method.study_with_options)
    /// passing the [`StudyExtraNeeded`](enum.StudyOption.html#variant.StudyExtraNeeded) study option.
    ///
    /// # Return value
    /// `true` if the use of the mark field could be enabled. `false` otherwise, which signifies
    /// that an extra block needs to be created.
    pub fn enable_mark(&mut self) -> bool {
        unsafe {
            if self.extra.is_null() {
                false
            } else {
                (*self.extra).set_mark(&mut self.mark_);
                true
            }
        }
    }

    /// Returns the extra block, if one has been created.
    pub fn extra(&mut self) -> Option<&mut PcreExtra> {
        unsafe {
            if self.extra.is_null() {
                None
            } else {
                Some(&mut *(self.extra))
            }
        }
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
    /// speed up matching. See the [study()](#method.study) method.
    #[inline]
    pub fn exec<'a, 'p>(&'p mut self, subject: &'a str) -> Option<Match<'a>> {
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
    /// speed up matching. See the [study()](#method.study) method.
    #[inline]
    pub fn exec_from<'a, 'p>(&'p mut self, subject: &'a str, startoffset: usize) -> Option<Match<'a>> {
        let no_options: EnumSet<ExecOption> = EnumSet::new();
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
    /// speed up matching. See the [study()](#method.study) method.
    #[inline]
    pub fn exec_from_with_options<'a, 'p>(&'p mut self, subject: &'a str, startoffset: usize, options: &EnumSet<ExecOption>) -> Option<Match<'a>> {
        let ovecsize = (self.capture_count_ + 1) * 3;
        let mut ovector = vec![0 as c_int; ovecsize as usize];

        unsafe {
            let rc = detail::pcre_exec(self.code,
                                       self.extra as *const PcreExtra,
                                       subject.as_ptr() as *const c_char,
                                       subject.len() as c_int,
                                       startoffset as c_int,
                                       options,
                                       ovector.as_mut_ptr(),
                                       ovecsize as c_int);
            if rc >= 0 {
                Some(Match {
                    subject: subject,
                    partial_ovector: ovector[..(((self.capture_count_ + 1) * 2) as usize)].to_vec(),
                    string_count_: rc
                })
            } else {
                None
            }
        }
    }

    /// Returns the mark name from PCRE if set.
    ///
    /// # Return value
    /// `Some(str)` if PCRE returned a value for the mark.
    /// `None` if either there was no mark set or [enable_mark()](#method.enable_mark) was not called,
    /// or was unsuccessful.
    #[inline]
    pub fn mark(&self) -> Option<String> {
        self.mark_bytes().map (|bytes| String::from_utf8(Vec::from(bytes)).unwrap())
    }

    /// Returns the mark name from PCRE if set.
    ///
    /// # Return value
    /// `Some(&[u8])` if PCRE returned a value for the mark.
    /// `None` if either there was no mark set or [enable_mark()](#method.enable_mark) was not called,
    /// or was unsuccessful.
    #[inline]
    pub fn mark_bytes(&self) -> Option<&[u8]> {
        unsafe {
            if self.mark_.is_null() {
                None
            } else {
                let mark_cstr = CStr::from_ptr(self.mark_ as *const c_char);
                Some(mark_cstr.to_bytes())
            }
        }
    }

    /// Creates a `MatchIterator` for iterating through matches within the given subject
    /// string `subject`.
    ///
    /// # Argument
    /// * `subject` - The subject string.
    #[inline]
    pub fn matches<'a, 'p>(&'p mut self, subject: &'a str) -> MatchIterator<'a, 'p> {
        let no_options: EnumSet<ExecOption> = EnumSet::new();
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
    pub fn matches_with_options<'a, 'p>(&'p mut self, subject: &'a str, options: &EnumSet<ExecOption>) -> MatchIterator<'a, 'p> {
        unsafe {
            let ovecsize = (self.capture_count_ + 1) * 3;
            MatchIterator {
                code: { detail::pcre_refcount(self.code as *mut detail::pcre, 1); self.code },
                extra: self.extra as *const PcreExtra,
                capture_count: self.capture_count_,
                subject: subject,
                offset: 0,
                options: options.clone(),
                ovector: vec![0 as c_int; ovecsize as usize],
                _marker: PhantomData
            }
        }
    }

    /// Returns the number of named capture groups in the regular expression.
    pub fn name_count(&self) -> usize {
        unsafe {
            let mut name_count: c_int = 0;
            detail::pcre_fullinfo(self.code, self.extra as *const PcreExtra, libpcre_sys::PCRE_INFO_NAMECOUNT, &mut name_count as *mut c_int as *mut c_void);
            name_count as usize
        }
    }

    /// Creates a name-to-number translation table that maps the name of each named capture
    /// group to the assigned group numbers.
    ///
    /// The value type of the returned `BTreeMap` is a `usize` vector because there can be
    /// more than one group number for a given name if the PCRE_DUPNAMES option is used
    /// when compiling the regular expression.
    pub fn name_table(&self) -> BTreeMap<String, Vec<usize>> {
        unsafe {
            let name_count = self.name_count();
            let mut tabptr: *const c_uchar = ptr::null();
            detail::pcre_fullinfo(self.code, self.extra as *const PcreExtra, libpcre_sys::PCRE_INFO_NAMETABLE, &mut tabptr as *mut *const c_uchar as *mut c_void);
            let mut name_entry_size: c_int = 0;
            detail::pcre_fullinfo(self.code, self.extra as *const PcreExtra, libpcre_sys::PCRE_INFO_NAMEENTRYSIZE, &mut name_entry_size as *mut c_int as *mut c_void);

            let mut name_table: BTreeMap<String, Vec<usize>> = BTreeMap::new();

            let mut i = 0;
            while i < name_count {
                let n: usize = ((ptr::read(tabptr) as usize) << 8) | (ptr::read(tabptr.offset(1)) as usize);
                let name_cstr = CStr::from_ptr(tabptr.offset(2) as *const c_char);
                // TODO Check memory allocations
                let name: String = String::from_utf8(Vec::from(name_cstr.to_bytes())).unwrap();
                // TODO Avoid the double lookup.
                // https://github.com/mozilla/rust/issues/9068
                if !name_table.contains_key(&name) {
                    name_table.insert(name, vec![n]);
                } else {
                    name_table.get_mut(&name).unwrap().push(n);
                }
                tabptr = tabptr.offset(name_entry_size as isize);
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
        let no_options: EnumSet<StudyOption> = EnumSet::new();
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
    /// `true` if additional information could be extracted or the [`StudyExtraNeeded`](enum.StudyOption.html#variant.StudyExtraNeeded)
    /// option was passed. `false` otherwise.
    pub fn study_with_options(&mut self, options: &EnumSet<StudyOption>) -> bool {
        unsafe {
            // If something else has a reference to `code` then it probably has a pointer to
            // the current study data (if any). Thus, we shouldn't free the current study data
            // in that case.
            if detail::pcre_refcount(self.code as *mut detail::pcre, 0) != 1 {
                false
            } else {
                // Free any current study data.
                detail::pcre_free_study(self.extra as *mut PcreExtra);
                self.extra = ptr::null_mut();

                let extra = detail::pcre_study(self.code, options);
                self.extra = extra;
                !extra.is_null()
            }
        }
    }
}

impl Drop for Pcre {
    fn drop(&mut self) {
        unsafe {
            if detail::pcre_refcount(self.code as *mut detail::pcre, -1) == 0 {
                detail::pcre_free_study(self.extra as *mut PcreExtra);
                detail::pcre_free(self.code as *mut detail::pcre as *mut c_void);
            }
            self.extra = ptr::null_mut();
            self.code = ptr::null();
        }
    }
}

impl<'a> Match<'a> {
    /// Returns the start index within the subject string of capture group `n`.
    ///
    /// If the capture group is present in the pattern but wasn't captured then the start of it will be `usize::max_value()`.
    /// Happens with the optional groups, `/(optional)?/`.
    pub fn group_start(&self, n: usize) -> usize {
        self.partial_ovector[(n * 2) as usize] as usize
    }

    /// Returns the end index within the subject string of capture group `n`.
    ///
    /// If the capture group is present in the pattern but wasn't captured then the end of it will be `usize::max_value()`.
    /// Happens with the optional groups, `/(optional)?/`.
    pub fn group_end(&self, n: usize) -> usize {
        self.partial_ovector[(n * 2 + 1) as usize] as usize
    }

    /// Returns the length of the substring for capture group `n`.
    pub fn group_len(&self, n: usize) -> usize {
        let group_offsets = &self.partial_ovector[((n * 2) as usize)..];
        (group_offsets[1] - group_offsets[0]) as usize
    }

    /// Returns the substring for capture group `n` as a slice.
    #[inline]
    pub fn group(&self, n: usize) -> &'a str {
        let group_offsets = &self.partial_ovector[((n * 2) as usize)..];
        let start = group_offsets[0];
        let end = group_offsets[1];
        &self.subject[(start as usize)..(end as usize)]
    }

    /// Returns the number of substrings captured.
    pub fn string_count(&self) -> usize {
        self.string_count_ as usize
    }
}

impl<'a, 'p> Drop for MatchIterator<'a, 'p> {
    fn drop(&mut self) {
        unsafe {
            if detail::pcre_refcount(self.code as *mut detail::pcre, -1) == 0 {
                detail::pcre_free_study(self.extra as *mut PcreExtra);
                detail::pcre_free(self.code as *mut detail::pcre as *mut c_void);
            }
            self.extra = ptr::null();
            self.code = ptr::null();
        }
    }
}

impl<'a, 'p> Iterator for MatchIterator<'a, 'p> {
    type Item = Match<'a>;

    /// Gets the next match.
    #[inline]
    fn next(&mut self) -> Option<Match<'a>> {
        unsafe {
            let rc = detail::pcre_exec(self.code,
                                       self.extra,
                                       self.subject.as_ptr() as *const c_char,
                                       self.subject.len() as c_int,
                                       self.offset,
                                       &self.options,
                                       self.ovector.as_mut_ptr(),
                                       self.ovector.len() as c_int);
            if rc >= 0 {
                // Update the iterator state.
                self.offset = self.ovector[1];

                Some(Match {
                    subject: self.subject,
                    partial_ovector: self.ovector[..(((self.capture_count + 1) * 2) as usize)].to_vec(),
                    string_count_: rc
                })
            } else {
                None
            }
        }
    }
}

/// Returns libpcre version information.
pub fn pcre_version() -> String {
    detail::pcre_version()
}
