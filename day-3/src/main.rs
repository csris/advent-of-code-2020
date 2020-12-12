#[macro_use]
extern crate clap;

use std::io;
use std::io::prelude::*;
use std::fs;

#[derive(Debug)]
struct TreeMap(Vec<Vec<bool>>);

impl TreeMap {
    fn load(file: &str) -> io::Result<TreeMap> {
        let f = fs::File::open(file)?;
        let reader = io::BufReader::new(f); 
        let trees = reader.lines().map(|line| {
            line.unwrap().chars().map(|ch| ch == '#').collect::<Vec<_>>()
        }).collect::<Vec<Vec<_>>>();

        Ok(TreeMap(trees))
    }

    fn at(&self, x: usize, y: usize) -> bool {
        let row = &self.0[y];
        row[x % row.len()]
    }

    fn rows(&self) -> usize {
        self.0.len()
    }
}

fn main() -> io::Result<()> {
    let matches = clap_app!(day_3 =>
        (version: "1.0")
        (author: "Charles Srisuwananukorn <csrisuw@gmail.com>")
        (about: "advent of code day 3")
        (@arg right: -r --right [N] "right component of the slope")
        (@arg down: -d --down [N] "down component of the slope")
        (@arg INPUT: +required "input file")
    ).get_matches();

    let right = matches.value_of("right").unwrap_or("3").parse::<usize>().unwrap();
    let down = matches.value_of("down").unwrap_or("1").parse::<usize>().unwrap();

    let map = TreeMap::load(matches.value_of("INPUT").unwrap())?;

    let mut count = 0;
    let mut i = 0;

    loop {
        let row = i * down;
        let col = i * right;

        if row >= map.rows() {
            break;
        }

        if map.at(col, row) {
            count = count + 1;
        }

        i = i + 1;
    }

    println!("count: {}", count);

    Ok(())
}
