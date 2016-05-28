// vim:set ft=rust fenc=utf-8 si sw=4:

extern crate getopts;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn prettify(file: &mut File) {
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(_) => panic!(""), 
        Ok(_) => print!(""),
    }
}

pub fn uumain(args: Vec<String>) -> i32 {
    let mut opts = getopts::Options::new();

    opts.optflag("v", "version", "show the version information");

    let matches = match opts.parse(&args[1..]) {
	Ok(m) => m, 
	Err(f) => {
	    println!("Invalid options\n{}", f);
	    return 1;
	}
    };

    if matches.opt_present("version") {
	println!("version {}", env!("CARGO_PKG_VERSION"));
	return 0;
    }

    for file in matches.free {
	let path = Path::new(&file);
	let mut file = match File::open(&path) {
	    Err(why) => {
		println!("Cannot open {} {}", path.display(), Error::description(&why));
		continue;
	    }, 
	    Ok(file) => file,
	};

	prettify(&mut file);
    }

    return 0;
}

