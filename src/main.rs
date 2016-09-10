extern crate getopts;

use getopts::{Matches, Options};
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, stdin, stderr};

struct CountResults {
    bytes: u64,
    chars: u64,
    lines: u64,
    words: u64,
    max_line_length: u64,
}

/// Read a file (via the given reader) line by line and count things.
fn do_count<R: Read>(reader: BufReader<R>) -> CountResults {
    let mut res = CountResults { bytes: 0, chars: 0, lines: 0,
                                 words: 0, max_line_length: 0 };

    for line in reader.lines() {
        let line = line.expect("There was an IO error reading a line.");

        res.lines += 1;
        res.bytes += line.len() as u64;
        res.bytes += 1; // don't forget the \n!

        res.words += line.split_whitespace().count() as u64;

        let length = line.chars().count() as u64;
        res.chars += length;
        res.chars += 1; // don't forget the \n!

        if length > res.max_line_length {
            res.max_line_length = length;
        }
    }

    res
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();

    opts.optflag("l", "lines", "print the newline counts");
    opts.optflag("w", "words", "print the word counts");
    opts.optflag("c", "bytes", "print the byte counts");
    opts.optflag("m", "chars", "print the character counts");
    opts.optflag("L",
                 "max-line-length",
                 "print the length of the longest line");
    opts.optflag("h", "help", "display this help and exit");

    let matches = opts.parse(&args[1..]).expect("Error parsing args");

    if matches.opt_present("h") {
        print_usage(&args[0], opts); // Like in C, args[0] is the program name
        return;
    }

    // If we have no args, read from stdin.
    if matches.free.is_empty() {
        let counts = do_count(BufReader::new(stdin()));
        print_results(&counts, &matches, "<stdin>");
    }

    // Otherwise...
    for filename in &matches.free {
        let file = match File::open(&filename) {
            Ok(f) => f,
            Err(e) => {
                writeln!(&mut stderr(), "{}: {}", filename, e).unwrap();
                continue;
            }
        };
        let counts = do_count(BufReader::new(file));
        print_results(&counts, &matches, filename);
    }
}

/// Given some counts, the parsed command-line args, and the filename,
/// print a line with the desired information.
fn print_results(counts: &CountResults, matches: &Matches, filename: &str) {
    if matches.opt_present("lines") { print!("{} ", counts.lines); }

    if matches.opt_present("words") { print!("{} ", counts.words) }

    if matches.opt_present("bytes") { print!("{} ", counts.bytes) }

    if matches.opt_present("chars") { print!("{} ", counts.chars) }

    if matches.opt_present("max-line-length") {
        print!("{} ", counts.max_line_length)
    }

    println!("{}", filename);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} rwc [options] [<file>]", program);
    print!("{}", opts.usage(&brief));
}
