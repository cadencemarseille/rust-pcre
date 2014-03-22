// Copyright 2014 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This is a port of the pcre project's `pcredemo` sample using rust-pcre bindings.

extern crate collections;
//extern crate extra;
extern crate getopts;
extern crate pcre;

use collections::treemap::{TreeMap};
use collections::enum_set::{EnumSet};
use getopts::{OptGroup, getopts, optflag};
use pcre::{CompileOption, Match, Pcre, pcre_version};
use std::io::stdio::stderr;
use std::os;

fn print_usage(program: &str, opts: &[OptGroup]) {
    drop(opts);
    println!("Usage: {} [options] pattern subject", program);
    println!("Options:");
    println!("    -g                  Find all matches");
    println!("    -h, --help          Print usage and exit");
    println!("    --version           Print version information and exit");
}

fn print_version_info() {
    println!("rust-pcre 0.1 compiled against libpcre {}", pcre_version());
}

fn print_match(m: &Match, name_table: &TreeMap<~str, ~[uint]>) {
    println!("Match succeeded at offset {:u}", m.group_start(0u));

    // Show captured substrings by number.
    let mut i = 0u;
    while i < m.string_count() {
        println!("{:2u}: {:s}", i, m.group(i));
        i += 1;
    }

    let name_count = name_table.len();
    if name_count <= 0 {
        println!("No named substrings");
    } else {
        println!("Named substrings:");
        for (name, n_vec) in name_table.iter() {
            for n in n_vec.iter() {
                println!("({:u}) {:s}: {:s}", *n, *name, m.group(*n));
            }
        }
    }
}

fn main() {
    let args = os::args();
    let program = args[0].clone();

    let opts = ~[
        optflag("g", "", "find all matches"),
        optflag("h", "help", "print usage and exit"),
        optflag("", "version", "print version information and exit")
    ];

    let opt_matches = match getopts(args.tail(), opts) {
        Ok(m)  => m,
        Err(f) => {
            stderr().write_line(format!("Error: {}", f.to_err_msg()));
            os::set_exit_status(1);
            return;
        }
    };

    if opt_matches.opt_present("h") || opt_matches.opt_present("help") {
        print_usage(program, opts);
        return;
    }

    if opt_matches.opt_present("version") {
        print_version_info();
        return;
    }

    let find_all = opt_matches.opt_present("g");
    if opt_matches.free.len() == 0 {
        stderr().write_line("Error: No pattern");
        os::set_exit_status(1);
        return;
    } else if opt_matches.free.len() == 1 {
        stderr().write_line("Error: No subject");
        os::set_exit_status(1);
        return;
    } else if opt_matches.free.len() > 2 {
        stderr().write_line("Error: Too many command line arguments");
        os::set_exit_status(1);
        return;
    }

    let pattern = opt_matches.free.get(0).clone();
    let subject = opt_matches.free.get(1).clone();

    let mut compile_options: EnumSet<CompileOption> = EnumSet::empty();
    compile_options.add(pcre::DupNames);
    let re = match Pcre::compile_with_options(pattern, &compile_options) {
        Err(err) => {
            stderr().write_line(format!("Error: The pattern could not be compiled: {:s}", err.to_str()));
            os::set_exit_status(1);
            return;
        },
        Ok(re) => re
    };
    let name_table = re.name_table();

    let opt_m = re.exec(subject);
    let m = match opt_m {
        None => {
            println!("No match");
            os::set_exit_status(1);
            return;
        }
        Some(m) => m
    };
    print_match(&m, &name_table);

    if find_all {
        let mut start_offset = m.group_end(0);
        loop {
            let opt_m = re.exec_from(subject, start_offset);
            let m = match opt_m {
                None => {
                    println!("\nNo more matches");
                    return;
                }
                Some(m) => m
            };

            println!("");
            print_match(&m, &name_table);

            start_offset = m.group_end(0);
        }
    }
}
