// Copyright 2015 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;

use libc::{c_char, c_int, c_uchar, c_ulong, c_void};
use std::option::{Option};
use std::ptr;

#[allow(non_camel_case_types)]
pub type compile_options = c_int;
#[allow(non_camel_case_types)]
pub type exec_options = c_int;
#[allow(non_camel_case_types)]
pub type fullinfo_field = c_int;
#[allow(non_camel_case_types)]
pub type study_options = c_int;

pub const PCRE_UTF8: compile_options = 0x00000800;

// PCRE_NO_UTF8_CHECK is both a compile and exec option
pub const PCRE_NO_UTF8_CHECK: c_int = 0x00002000;

pub const PCRE_ERROR_NOMATCH: c_int = -1;
pub const PCRE_ERROR_NULL: c_int = -2;

pub const PCRE_INFO_CAPTURECOUNT: fullinfo_field = 2;
pub const PCRE_INFO_NAMEENTRYSIZE: fullinfo_field = 7;
pub const PCRE_INFO_NAMECOUNT: fullinfo_field = 8;
pub const PCRE_INFO_NAMETABLE: fullinfo_field = 9;

//const PCRE_EXTRA_STUDY_DATA: c_ulong = 0x0001;
const PCRE_EXTRA_MATCH_LIMIT: c_ulong = 0x0002;
//const PCRE_EXTRA_CALLOUT_DATA: c_ulong = 0x0004;
//const PCRE_EXTRA_TABLES: c_ulong = 0x0008;
const PCRE_EXTRA_MATCH_LIMIT_RECURSION: c_ulong = 0x0010;
const PCRE_EXTRA_MARK: c_ulong = 0x0020;
//const PCRE_EXTRA_EXECUTABLE_JIT: c_ulong = 0x0040;

#[allow(non_camel_case_types)]
pub enum pcre {}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct pcre_extra {
    flags: c_ulong,
    study_data: *mut c_void,
    match_limit_: c_ulong,
    callout_data: *mut c_void,
    tables: *const c_uchar,
    match_limit_recursion_: c_ulong,
    mark: *mut *mut c_uchar,
    executable_jit: *mut c_void
}

impl pcre_extra {
    /// Returns the match limit, if previously set by [set_match_limit()](#method.set_match_limit).
    ///
    /// The default value for this limit is set when PCRE is built. The default default is 10 million.
    pub fn match_limit(&self) -> Option<usize> {
        if (self.flags & PCRE_EXTRA_MATCH_LIMIT) == 0 {
            None
        } else {
            Some(self.match_limit_ as usize)
        }
    }

    /// Sets the match limit to `limit` instead of using PCRE's default.
    pub fn set_match_limit(&mut self, limit: u32) {
        self.flags |= PCRE_EXTRA_MATCH_LIMIT;
        self.match_limit_ = limit as c_ulong;
    }

    /// Returns the recursion depth limit, if previously set by [set_match_limit_recursion()](#method.set_match_limit_recursion).
    ///
    /// The default value for this limit is set when PCRE is built.
    pub fn match_limit_recursion(&self) -> Option<usize> {
        if (self.flags & PCRE_EXTRA_MATCH_LIMIT_RECURSION) == 0 {
            None
        } else {
            Some(self.match_limit_recursion_ as usize)
        }
    }

    /// Sets the recursion depth limit to `limit` instead of using PCRE's default.
    pub fn set_match_limit_recursion(&mut self, limit: u32) {
        self.flags |= PCRE_EXTRA_MATCH_LIMIT_RECURSION;
        self.match_limit_ = limit as c_ulong;
    }

    /// Sets the mark field.
    pub unsafe fn set_mark(&mut self, mark: &mut *mut c_uchar) {
        self.flags |= PCRE_EXTRA_MARK;
        self.mark = mark as *mut *mut c_uchar;
    }

    /// Unsets the mark field. PCRE will not save mark names when matching the compiled regular expression.
    pub fn unset_mark(&mut self) {
        self.flags &= !PCRE_EXTRA_MARK;
        self.mark = ptr::null_mut();
    }
}

#[link(name = "pcre")]
extern {
    pub static pcre_free: extern "C" fn(ptr: *mut c_void);

    pub fn pcre_compile(pattern: *const c_char, options: compile_options, errptr: *mut *const c_char, erroffset: *mut c_int, tableptr: *const c_uchar) -> *mut pcre;
    pub fn pcre_exec(code: *const pcre, extra: *const pcre_extra, subject: *const c_char, length: c_int, startoffset: c_int, options: exec_options, ovector: *mut c_int, ovecsize: c_int) -> c_int;
    pub fn pcre_free_study(extra: *mut pcre_extra);
    pub fn pcre_fullinfo(code: *const pcre, extra: *const pcre_extra, what: fullinfo_field, where_: *mut c_void) -> c_int;
    // Note: libpcre's pcre_refcount() function is not thread-safe.
    pub fn pcre_refcount(code: *mut pcre, adjust: c_int) -> c_int;
    pub fn pcre_study(code: *const pcre, options: study_options, errptr: *mut *const c_char) -> *mut pcre_extra;
    pub fn pcre_version() -> *const c_char;
}
