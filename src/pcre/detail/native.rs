// Copyright 2014 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::{c_char, c_int, c_uchar, c_void};

#[link(name = "pcre")]
extern {
    pub static pcre_free: extern "C" fn(ptr: *mut c_void);

    pub fn pcre_compile(pattern: *const c_char, options: ::detail::compile_options, errptr: *mut *const c_char, erroffset: *mut c_int, tableptr: *const c_uchar) -> *mut ::detail::pcre;
    pub fn pcre_exec(code: *const ::detail::pcre, extra: *const ::PcreExtra, subject: *const c_char, length: c_int, startoffset: c_int, options: ::detail::exec_options, ovector: *mut c_int, ovecsize: c_int) -> c_int;
    pub fn pcre_free_study(extra: *mut ::PcreExtra);
    pub fn pcre_fullinfo(code: *const ::detail::pcre, extra: *const ::PcreExtra, what: ::detail::fullinfo_field, where: *mut c_void) -> c_int;
    // Note: libpcre's pcre_refcount() function is not thread-safe.
    pub fn pcre_refcount(code: *mut ::detail::pcre, adjust: c_int) -> c_int;
    pub fn pcre_study(code: *const ::detail::pcre, options: ::detail::study_options, errptr: *mut *const c_char) -> *mut ::PcreExtra;
    pub fn pcre_version() -> *const c_char;
}
