// parse million-dollar-words.txt

use std::{
    fmt::Display,
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
#[derive(Debug)]
struct Entry {
    pub term: String,
    pub definition: String,
}

impl Display for Entry {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        writeln!(fmt, "### {}\n{}\n", self.term, self.definition)
    }
}

// ---

fn main() {
    // let lines = lines_from_file("./million-dollar-words.txt");
    let lines = lines_from_file("/Users/Steve/Dropbox/Books/million-dollar-words.txt");
    let entries = lines.split(|l| l == "");
    let mut dictionary: Vec<Entry> = Vec::new();

    for entry in entries {
        let word = &entry[0];
        let def = entry[1..].join(" ");

        let term = Entry {
            term: word.to_string(),
            definition: def,
        };
        dictionary.push(term);
    }

    // print the dictionary
    for word in dictionary.iter() {
        println!("{}", word);
    }
}
