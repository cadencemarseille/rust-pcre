// Copyright 2015 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This is a port of the pcre project's `pcredemo` sample using rust-pcre bindings.

extern crate enum_set;
extern crate getopts;
extern crate pcre;

use enum_set::{EnumSet};
use getopts::{Options};
use pcre::{CompileOption, Match, Pcre, pcre_version};
use std::collections::{BTreeMap};
use std::env;
use std::io::{stderr, Write};
use std::string::{String};
use std::vec::{Vec};

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] pattern subject", program);
    print!("{}", opts.usage(&brief));
}

fn print_version_info() {
    println!("rust-pcre 0.2.3 compiled against libpcre {}", pcre_version());
}

fn print_match(m: &Match, name_table: &BTreeMap<String, Vec<usize>>) {
    println!("Match succeeded at offset {}", m.group_start(0));

    // Show captured substrings by number.
    let mut i = 0;
    while i < m.string_count() {
        println!("{}: {}", i, m.group(i));
        i += 1;
    }

    let name_count = name_table.len();
    if name_count <= 0 {
        println!("No named substrings");
    } else {
        println!("Named substrings:");
        for (name, n_vec) in name_table.iter() {
            for n in n_vec.iter() {
                println!("({}) {}: {}", *n, *name, m.group(*n));
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("g", "", "find all matches");
    opts.optflag("h", "help", "print usage and exit");
    opts.optflag("", "version", "print version information and exit");
    let opt_matches = match opts.parse(&args[1..]) {
        Ok(m)  => m,
        Err(f) => {
            writeln!(stderr(), "Error: {}", f).unwrap();
            //env::set_exit_status(1);
            return;
        }
    };

    if opt_matches.opt_present("h") || opt_matches.opt_present("help") {
        print_usage(&program, &opts);
        return;
    }

    if opt_matches.opt_present("version") {
        print_version_info();
        return;
    }

    let find_all = opt_matches.opt_present("g");
    if opt_matches.free.len() == 0 {
        writeln!(stderr(), "Error: No pattern").unwrap();
        //env::set_exit_status(1);
        return;
    } else if opt_matches.free.len() == 1 {
        writeln!(stderr(), "Error: No subject").unwrap();
        //env::set_exit_status(1);
        return;
    } else if opt_matches.free.len() > 2 {
        writeln!(stderr(), "Error: Too many command line arguments").unwrap();
        //env::set_exit_status(1);
        return;
    }

    let pattern = opt_matches.free[0].clone();
    let subject = opt_matches.free[1].clone();

    let mut compile_options: EnumSet<CompileOption> = EnumSet::new();
    compile_options.insert(CompileOption::DupNames);
    let mut re = match Pcre::compile_with_options(&pattern, &compile_options) {
        Err(err) => {
            writeln!(stderr(), "Error: The pattern could not be compiled: {}", err).unwrap();
            //env::set_exit_status(1);
            return;
        },
        Ok(re) => re
    };
    let name_table = re.name_table();

    let opt_m = re.exec(&subject);
    let m = match opt_m {
        None => {
            println!("No match");
            //env::set_exit_status(1);
            return;
        }
        Some(m) => m
    };
    print_match(&m, &name_table);

    if find_all {
        let mut start_offset = m.group_end(0);
        loop {
            let opt_m = re.exec_from(&subject, start_offset);
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
