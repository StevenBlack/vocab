// parse million-dollar-words.txt

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// ---

fn main() {
    let lines = lines_from_file("./million-dollar-words.txt");
    let entries = lines.split(|l| l == "");

    for entry in entries {
        let term = &entry[0];
        let def = entry[1..].join(" ");

        println!("Term: {:?}", term);
        println!("Definition: {:?}", def);
    }
}
