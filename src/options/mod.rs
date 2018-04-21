extern crate getopts;
use getopts::Options;
use getopts::Matches;

pub fn print_usage(program: &str, opts: Options) {
    let brief = format!(
        "Usage: {} [OPTION]... [FILE]... \n
        List information about the FILEs (the current directory by default). \
        Sort entries alphabetically if none of -cftuvSUX nor --sort. \
        \
        Mandatory arguments to long options are mandatory for short options too.", program);
    print!("{}", opts.usage(&brief));
    print!("\n\
            SIZE may be (or may be an integer optionally followed by) one of following: \n\
            kB 1000, K 1024, MB 1000*1000, M 1024*1024, and so on for G, T, P, E, Z, Y. \n\
            By default, color is not used to distinguish types of files.  That is \n\
            equivalent to using --color=none.  Using the --color option without the \n\
            optional WHEN argument is equivalent to using --color=always.  With \n\
            --color=auto, color codes are output only if standard output is connected \n\
            to a terminal (tty).  The environment variable LS_COLORS can influence the \n\
            colors, and can be set easily by the dircolors command. \n\
            \n\
            Exit status is 0 if OK, 1 if minor problems, 2 if serious trouble. \n\
            \n\
            Report bugs to <co1row.jp@gmail.com>.\n");
}

pub fn make_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("a", "all", "do not ignore entries starting with .");
    opts.optflag("A", "almost-all", "do not list implied . and ..");
    // opts.optflag("", "author", "with -l, print the author of each file");
    // opts.optflag("b", "escape", "print octal escapes for nongraphic characters");
    // opts.optflag("", "block-size=SIZE", "use SIZE-byte blocks");
    // opts.optflag("B", "ignore-backups", "do not list implied entries ending with ~");
    // opts.optflag("c", "", " with -lt: sort by, and show, ctime (time of last \
    //                        modification of file status information) \
    //                        with -l: show ctime and sort by name \
    //                        otherwise: sort by ctime");
    // opts.optflag("C", "", "control whether color is used to distinguish file \
    //                         types.  WHEN may be `never', `always', or `auto'");
    // opts.optflag("", "color[=WHEN]", "(ignored)");
    // opts.optflag("d", "directory", "list directory entries instead of contents, \
    //                                 and do not dereference symbolic links");
    // opts.optflag("D", "dired", "generate output designed for Emacs' dired mode");
    // opts.optflag("f", "", "do not sort, enable -aU, disable -lst");
    // opts.optflag("F", "classify", "append indicator (one of */=>@|) to entries");
    // opts.optflag("", "file-type", "likewise, except do not append `*'");
    // opts.optflag("", "format=WORD", "across -x, commas -m, horizontal -x, long -l, \
    //                                 single-column -1, verbose -l, vertical -C");
    // opts.optflag("", "full-time", "like -l --time-style=full-iso");
    // opts.optflag("g", "", "like -l, but do not list owner");
    // opts.optflag("G", "no-group", "like -l, but do not list group");
    // opts.optflag("h", "human-readable", "with -l, print sizes in human readable format \
    //                                     (e.g., 1K 234M 2G)");
    // opts.optflag("", "si", "likewise, but use powers of 1000 not 1024");
    // opts.optflag("H", "", "dereference-command-line \
    //                       follow symbolic links listed on the command line");
    // opts.optflag("", "dereference-command-line-symlink-to-dir",
    //                      "follow each command line symbolic link \
    //                         that points to a directory");
    // opts.optflag("", "hide=PATTERN", "do not list implied entries matching shell PATTERN \
    //                                    (overridden by -a or -A)");
    // opts.optflag("", "indicator-style=WORD", "append indicator with style WORD to entry names: \
    //                                             none (default), slash (-p), \
    //                                             file-type (--file-type), classify (-F)");
    // opts.optflag("i", "inode", "print the index number of each file");
    // opts.optflag("I", "ignore=PATTERN", "do not list implied entries matching shell PATTERN");
    // opts.optflag("k", "", "like --block-size=1K");
    // opts.optflag("l", "", "use a long listing format");
    // opts.optflag("L", "dereference", "when showing file information for a symbolic \
    //                                     link, show information for the file the link \
    //                                     references rather than for the link itself");
    // opts.optflag("m", "", "fill width with a comma separated list of entries");
    // opts.optflag("n", "numeric-uid-gid", "like -l, but list numeric user and group IDs");
    // opts.optflag("N", "literal", "print raw entry names (don't treat e.g. control \
    //                                 characters specially)");
    // opts.optflag("o", "", "like -l, but do not list group information");
    // opts.optflag("p", "indicator-style=slash", "append / indicator to directories");
    // opts.optflag("q", "hide-control-chars", "print ? instead of non graphic characters");
    // opts.optflag("", "show-control-chars", "show non graphic characters as-is (default \
    //                                         unless program is `ls' and output is a terminal)");
    // opts.optflag("Q", "quote-name", "enclose entry names in double quotes");
    // opts.optflag("", "quoting-style=WORD", "use quoting style WORD for entry names: \
    //                                         literal, locale, shell, shell-always, c, escape");
    // opts.optflag("r", "reverse", "reverse order while sorting");
    opts.optflag("R", "recursive", "list subdirectories recursively");
    // opts.optflag("s", "size", "with -l, print size of each file, in blocks");
    // opts.optflag("S", "", "sort by file size");
    // opts.optflag("", "sort=WORD", "extension -X, none -U, size -S, time -t, \
    //                                 version -v, status -c, time -t, atime -u, \
    //                                 access -u, use -u");
    // opts.optflag("", "time=WORD", "with -l, show time as WORD instead of modification \
    //                                 time: atime, access, use, ctime or status; use \
    //                                 specified time as sort key if --sort=time");
    // opts.optflag("", "time-style=STYLE", "with -l, show times using style STYLE: \
    //                                     full-iso, long-iso, iso, locale, +FORMAT. \
    //                                     FORMAT is interpreted like `date'; if FORMAT is \
    //                                     FORMAT1<newline>FORMAT2, FORMAT1 applies to \
    //                                     non-recent files and FORMAT2 to recent files; \
    //                                     if STYLE is prefixed with `posix-', STYLE \
    //                                     takes effect only outside the POSIX locale");
    // opts.optflag("t", "", "sort by modification time");
    // opts.optflag("T", "tabsize=COLS", "assume tab stops at each COLS instead of 8");
    // opts.optflag("u", "", "with -lt: sort by, and show, access time \
    //                        with -l: show access time and sort by name \
    //                        otherwise: sort by access time");
    // opts.optflag("U", "", "do not sort; list entries in directory order");
    // opts.optflag("v", "", "sort by version");
    // opts.optflag("w", "width=COLS", "assume screen width instead of current value");
    // opts.optflag("x", "", "list entries by lines instead of by columns");
    // opts.optflag("X", "", "sort alphabetically by entry extension");
    opts.optflag("1", "", "list one file per line");
    // opts.optflag("", "append-exe", "append .exe if cygwin magic was needed");
    opts.optflag("", "help", "display this help and exit");
    opts.optflag("", "version", "output version information and exit");    
    return opts;
}

#[derive(Debug)]
pub struct LsOptions {
    pub all: bool,
    pub almost_all: bool,
    // pub long_listing_format: bool,
    pub recursive: bool,
    pub one_file_per_line: bool,
}

impl LsOptions {
    pub fn new(matches: &Matches) -> LsOptions {
        LsOptions {
            all: matches.opt_present("a"),
            almost_all: matches.opt_present("A"),
            // long_listing_format: matches.opt_present("l"),
            recursive: matches.opt_present("R"),
            one_file_per_line: matches.opt_present("1"),
        }
    }
}