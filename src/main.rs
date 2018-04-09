extern crate getopts;
extern crate help;
use getopts::Options;
use getopts::Matches;
use std::env;
use std::fs;
use std::io::{BufReader, BufRead, Read};
use std::io;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = help::make_options();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("help") {
        help::print_usage(&program, opts);
        return;
    }

    if matches.opt_present("version") {
        println!("myls version 0.01");
        println!("written by co1row");
        return;
    }

    return;
}
