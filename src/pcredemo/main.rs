// Copyright 2013 The rust-pcre authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This is a port of the pcre project's `pcredemo` sample using rust-pcre bindings.

extern mod extra;
extern mod pcre;

use extra::getopts::*;
use pcre::*;
use std::hashmap::{HashMap};
use std::os;

fn print_usage(program: &str, opts: &[Opt]) {
    println!("Usage: {} [options] pattern subject", program);
    println("Options:");
    println("    -g                  Find all matches");
    println("    -h, --help          Print usage and exit");
    println("    --version           Print version information and exit");
}

fn print_version_info() {
    println!("rust-pcre 0.1 compiled against libpcre {}", pcre_version());
}

fn print_match(m: &Match, name_table: &HashMap<~str, ~[uint]>) {
    println(fmt!("Match succeeded at offset %u", m.group_start(0u)));

    // Show captured substrings by number.
    let mut i = 0u;
    while i < m.string_count() {
        println(fmt!("%2u: %s", i, m.group(i)));
        i += 1;
    }

    let name_count = name_table.len();
    if name_count <= 0 {
        println("No named substrings");
    } else {
        println("Named substrings:");
        for (name, n_vec) in name_table.iter() {
            for n in n_vec.iter() {
                println(fmt!("(%u) %s: %s", *n, *name, m.group(*n)));
            }
        }
    }
}

fn main() {
    let args = os::args();
    let program = args[0].clone();

    let opts = ~[
        optflag("g"),
        optflag("h"),
        optflag("help"),
        optflag("version")
    ];

    let opt_matches = match getopts(args.tail(), opts) {
        Ok(m)  => m,
        Err(f) => {
            println!("Error: {}", f.to_err_msg());
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
        println("Error: No pattern");
        os::set_exit_status(1);
        return;
    } else if opt_matches.free.len() == 1 {
        println("Error: No subject");
        os::set_exit_status(1);
        return;
    } else if opt_matches.free.len() > 2 {
        println("Error: Too many command line arguments");
        os::set_exit_status(1);
        return;
    }

    let pattern = opt_matches.free[0].clone();
    let subject = opt_matches.free[1].clone();

    let re = Pcre::compile_with_options(pattern, PCRE_DUPNAMES);
    let name_table = re.name_table();

    let opt_m = re.exec(subject);
    let m = match opt_m {
        None => {
            println("No match");
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
                    println("\nNo more matches");
                    return;
                }
                Some(m) => m
            };

            println("");
            print_match(&m, &name_table);

            start_offset = m.group_end(0);
        }
    }
}
