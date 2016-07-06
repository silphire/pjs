// vim:set ft=rust fenc=utf-8 si sw=4:

extern crate getopts;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn prettify(file: &mut File) {
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(_) => panic!("Cannot read from the file "), 
        Ok(_) => print!(""),
    }
    let pos = 0;
    println!("{}", parse_json(&contents, pos).as_str());
}

// JSON-text = ws value ws
fn parse_json(json: &String, index: i32) -> String {
    let mut prettified = String::new();
    return prettified;
}

// ws = *(%x20 / %x09 / %x0A / %x0D)
fn parse_whitespace(json: &String, index: i32) -> String {
    let prettified = String::new();
    return prettified;
}

// value = false / null / true / object / array / number / string
fn parse_value(json: &String, index: i32) -> String {
    let prettified = String::new();
    return prettified;
}

// object = '{' [ member *( ',' member ) ] '}'
fn parse_object(json: &String, index: i32) -> String {
    let prettified = String::new();
    return prettified;
}

// member = string ':' value
fn parse_member(json: &String, index: &i32) -> String {
    let prettified = String::new();
    return prettified;
}

// array = '[' [ value *(':' value) ] ']'
fn parse_array(json: &String, index: i32) -> String {
    let prettified = String::new();
    return prettified;
}

// number
fn parse_number(json: &String, index: i32) -> String {
    let prettified = String::new();
    return prettified;
}

// string = '"' *char '"'
fn parse_string(json: &String, index: i32) -> String {
    let prettified = String::new();
    return prettified;
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

    for filename in matches.free {
	let path = Path::new(&filename);
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

