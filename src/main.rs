extern crate getopts;
use std::fs::Metadata;
use std::env;
use std::path::Path;
use getopts::Matches;
mod options;
use options::LsOptions;

fn print_not_exists(paths: &Vec<&Path>) {
    for path in paths.iter().filter(|e| ! e.exists()) {
        println!("myls: {}: No such file or directory", path.display());
    }
}

fn print_file<P>(path_arg: P, ls_options: &LsOptions) 
    where P: AsRef<Path>
{
    let path = path_arg.as_ref();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    print_file_impl(file_name, None, ls_options);
}

fn print_file_impl(file_name: &str, _meta: Option<Metadata>, ls_options: &LsOptions) {
    if is_hidden_file(file_name, ls_options) {
        return;
    }
    print!("{}  ", file_name);   
}

fn is_hidden_file(file_name: &str, ls_options: &LsOptions) -> bool {
    return ! (ls_options.all || ls_options.almost_all) && file_name.starts_with(".");
}

fn print_dir<P>(path_arg: P, ls_options: &LsOptions, show_dir_name: bool) 
    where P: AsRef<Path>
{
    let path = path_arg.as_ref();
    if show_dir_name {
        println!("{}:", path.to_str().unwrap());
    }

    if ls_options.all {
        if let Ok(meta) = path.metadata() {
            print_file_impl(".", Some(meta), ls_options); // print current dir
        }
        if let Some(parent) = path.parent() {
            if let Ok(meta) = parent.metadata() {
                print_file_impl("..", Some(meta), ls_options); // print parent dir
            }
        }
    }
   
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            print_file(entry.path(), ls_options);
        }
    }
    print!("\n\n");

    if ls_options.recursive {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();

                if path.is_dir() && ! is_hidden_file(file_name, ls_options) {
                    print_dir(entry.path(), ls_options, true);
                }
            }
        }
    }
}

fn print(matches: &Matches) -> Result<(), String> {
    let ls_options = LsOptions::new(matches);
    if matches.free.len() == 0 {
        print_dir(Path::new("./"), &ls_options, false);
    } else {
        let argument_paths = matches.free.iter().map(|e| Path::new(e)).collect::<Vec<_>>();
        let show_dir_name = matches.free.len() > 1;
        print_not_exists(&argument_paths);  
        for n in argument_paths.iter().filter(|e| e.exists()) {
            if n.is_dir() {
                print_dir(n, &ls_options, show_dir_name);
                println!("");
            } else {
                print_file(n, &ls_options);
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
    
    match print(&matches) {
        Ok(()) => (),
        Err(e) => print!("{}", e),
    }
    return;
}
