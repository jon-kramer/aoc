use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_reader = BufReader::new(File::open("../input").unwrap());
    // let output = part1(BufReader::new(stdin().lock()));
    // let output = part1(file_reader);
    let output = part1(file_reader);
}

fn part1<R>(reader: R) -> u32
where
    R: BufRead,
{
    let cards: Vec<_> = reader
        .lines()
        .map(|line| Card::from_line(&line.unwrap()))
        .collect();
}
