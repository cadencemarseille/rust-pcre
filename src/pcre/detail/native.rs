// Copyright 2013 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::libc::*;

#[link_args = "-lpcre.1"]
extern {
    static pcre_free: extern "C" fn(ptr: *c_void);

    fn pcre_compile(pattern: *c_char, options: ::options, errptr: *mut *c_char, erroffset: *mut c_int, tableptr: *c_uchar) -> *::detail::pcre;
    fn pcre_exec(code: *::detail::pcre, extra: *::detail::pcre_extra, subject: *c_char, length: c_int, startoffset: c_int, options: ::options, ovector: *mut c_int, ovecsize: c_int) -> ::detail::pcre_error;
    fn pcre_free_study(extra: *::detail::pcre_extra);
    fn pcre_fullinfo(code: *::detail::pcre, extra: *::detail::pcre_extra, what: ::detail::fullinfo_field, where: *mut c_void) -> c_int;
    fn pcre_study(code: *::detail::pcre, options: ::study_options, errptr: *mut *c_char) -> *::detail::pcre_extra;
}
