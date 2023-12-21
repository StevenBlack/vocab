// parse million-dollar-words.txt

use std::{
    fmt::Display,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use structopt::StructOpt;
use textwrap;
#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Just index?
    #[structopt(short, long)]
    index: bool,

    /// Plaintext output?
    #[structopt(long)]
    plain: bool,

    /// Incluse table of contents?
    #[structopt(short, long)]
    toc: bool,

    /// Wrap text and how wide?
    #[structopt(short, long)]
    wrap: Option<usize>,
}

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
    pub wrap: Option<usize>,
    pub plain: bool,
}

impl Display for Entry {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        if self.wrap.is_some() {
            if self.plain {
                writeln!(fmt, "{}\n{}\n", self.term, &textwrap::fill(&self.definition, self.wrap.unwrap()))
            } else {
                writeln!(fmt, "### {}\n{}\n", self.term, &textwrap::fill(&self.definition, self.wrap.unwrap()))
            }
        } else {
            if self.plain {
                writeln!(fmt, "{}\n{}\n", self.term, &self.definition)
            } else {
                writeln!(fmt, "### {}\n{}\n", self.term, &self.definition)
            }
        }
    }
}

#[derive(Debug)]
struct TOCEntry {
    pub term: String,
    pub letter: String,
}

type Dictionary = Vec<Entry>;
type TOC = Vec<TOCEntry>;

// ---
fn main() {
    let opt = Opt::from_args();
    let lines = lines_from_file("./vocabulary.txt");
    let entries = lines.split(|l| l == "");
    let mut dictionary: Dictionary = Vec::new();
    let mut toc: TOC = Vec::new();

    for entry in entries {
        if entry.len() == 0 {
            continue;
        }
        let word = &entry[0];
        let def = entry[1..].join(" ");

        let term = Entry {
            term: word.to_string(),
            definition: def,
            wrap: opt.wrap,
            plain: opt.plain,
        };
        dictionary.push(term);
    }

    // sort the dictionary
    dictionary.sort_by(|t1, t2| t1.term.to_lowercase().cmp(&t2.term.to_lowercase()));

    // build the table of contents
    if opt.toc {
        let mut letter = "".to_string();
        for entry in dictionary.iter() {
            let l = entry
                .term
                .chars()
                .next()
                .unwrap()
                .to_string()
                .to_lowercase();
            if letter != l {
                let newletter = l.to_string();
                letter = newletter;
                let term = TOCEntry {
                    term: entry.term.to_string(),
                    letter: l.to_string(),
                };
                toc.push(term);
            }
        }
    }

    // print the dictionary
    // prinr the index?
    if opt.index {
        for word in dictionary.iter() {
            println!("{}", word.term);
        }
    } else
    // build an print the dictionary
    {
        // print the toc
        if opt.toc && toc.len() > 0 {
            println!("## Contents\n\n");
            let mdtoc: Vec<String> = toc
                .iter()
                .map(|x| {
                    format!(
                        "[**{}**](#{})",
                        x.letter,
                        x.term.replace(" ", "-").to_lowercase()
                    )
                })
                .collect();

            println!("{}", mdtoc.join(" - ").to_string());
        }

        for word in dictionary.iter() {
            println!("{}", word);
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // ... tests go here ...
}
