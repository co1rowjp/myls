extern crate getopts;
use std::env;
use std::path::Path;
use std::fs;
use getopts::Matches;
mod options;

fn print_not_exists(paths: &Vec<&Path>) {
    for path in paths.iter().filter(|e| ! e.exists()) {
        println!("myls: {}: No such file or directory", path.display());
    }
}

fn print_file(f: &Path, matches: &Matches) {
    print!("{}  ", f.file_name().unwrap().to_str().unwrap());
}

fn print_dir(f: &Path, matches: &Matches, show_dir_name: bool) {
    if show_dir_name {
        println!("{}:", f.to_str().unwrap());
    }
    for n in fs::read_dir(f.to_str().unwrap()).unwrap() {
        print_file(n.unwrap().path().as_path(), matches);
    }
    println!("");
}

fn print(matches: &Matches) -> Result<(), String> {
    if matches.free.len() == 0 {
        for n in fs::read_dir("").unwrap() {
            print_file(n.unwrap().path().as_path(), matches);
        }
    } else {
        let argument_paths = matches.free.iter().map(|e| Path::new(e)).collect::<Vec<_>>();
        let show_dir_name = matches.free.len() > 1;
        print_not_exists(&argument_paths);
        for n in argument_paths.iter().filter(|e| e.exists()) {
            if n.is_dir() {
                print_dir(n, matches, show_dir_name);
                println!("");
            } else {
                print_file(n, matches);
            }
        }
    }
    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = options::make_options();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("help") {
        options::print_usage(&program, opts);
        return;
    }

    if matches.opt_present("version") {
        println!("myls version 0.01");
        println!("written by co1row");
        return;
    }
    print(&matches);
    return;
}
