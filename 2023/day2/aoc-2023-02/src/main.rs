use std::{collections::HashMap, io::stdin, u32};

use regex::Regex;

#[derive(Debug)]
struct Game {
    id: u32,
    reveal_sets: Vec<RevealSet>,
}

#[derive(Debug)]
struct RevealSet {
    // TODO: to make it easier to add colors,
    // instead make this a map whose keys are color names.
    // Kinda minor difference from just adding the names here..
    // but then no need to respecify u32, and the parser can
    // likewise be more generic.
    red: u32,
    blue: u32,
    green: u32,
}

// Take Games in the following format and parse into our data structure:
//  Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//
// Define our grammar using production rules in Extended Backus–Naur form
//  https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form
//
//  Game* = GameString, ":", Whitespace, RevealSets
//  GameString = "Game", " ", GameNumber
//  GameNumber = [0,9]+
//  RevealSets = (RevealSet (; RevealSet)*)
//  RevealSet = (ColorCount (, ColorCount)*)
//  ColorCount = [0,9]+, " ", Color
//  Color = red | green | blue
//
// To implement the parsing of this grammar there are several approaches:
// 1. Manual - hard code the parsing with string matches / regex
// 2. Write lexer + parser
// 3. Parser generator via Antlr4
//
// Current impl is 1.

fn main() {
    // part1();
    // part2();
    part3();
}

fn part1() {
    // TODO: take these as arguments?
    //  Perhaps a file mapping color to max counts
    //  Otherwise its not so flexible to add new colors
    let red_total = 12;
    let green_total = 13;
    let blue_total = 14;

    let mut valid_id_total = 0;

    'games: for line in stdin().lines() {
        // println!("{}", &line.as_ref().unwrap());
        let game = parse_game(&line.unwrap());
        // println!("{:#?}", game);
        // check each reveal
        for reveal in game.reveal_sets {
            if reveal.green > green_total || reveal.red > red_total || reveal.blue > blue_total {
                // Invalid reveal! skip this game and move on!
                continue 'games;
            }
        }
        // All reveals were valid, add the game id
        valid_id_total += game.id;
    }

    println!("{}", valid_id_total);
}

fn part2() {
    let mut power_total = 0;

    for line in stdin().lines() {
        // println!("{}", &line.as_ref().unwrap());
        let game = parse_game(&line.unwrap());
        // println!("{:#?}", game);

        // find max for each color across each reveal for this game
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for reveal in game.reveal_sets {
            if reveal.green > max_green {
                max_green = reveal.green;
            }
            if reveal.red > max_red {
                max_red = reveal.red;
            }
            if reveal.blue > max_blue {
                max_blue = reveal.blue;
            }
        }

        power_total += max_red * max_green * max_blue;
    }

    println!("{}", power_total);
}

fn part3() {
    let mut power_total = 0;

    for line in stdin().lines() {
        let color_count_map = parse_game_to_color_counts(&line.unwrap());

        power_total += color_count_map["green"] * color_count_map["red"] * color_count_map["blue"];
    }

    println!("{}", power_total);
}

// Regex time to parse game
// The individial reveals don't matter, instead just find all matching
// count-color segments and throw into a list
// Then turn the list into a map, where as we collect and find color
// key conflicts, we resolve by always taking the max count.
// These max values for each color can then be easily used to solve part 1 or 2.
fn parse_game_to_color_counts(text: &str) -> HashMap<String, u32> {
    let pattern = r"(\d+) (blue|red|green)";
    let re = Regex::new(pattern).expect("Failed to compile regex");

    let color_count_map = re
        .captures_iter(text)
        .fold(HashMap::new(), |mut map, captures| {
            let count = captures[1].parse::<u32>().unwrap();
            let color = captures[2].to_string();
            map.entry(color)
                .and_modify(|existing| {
                    if *existing < count {
                        *existing = count
                    }
                })
                .or_insert(count);
            map
        });
    color_count_map
}

fn parse_game(text: &str) -> Game {
    // split by : to get GameString and RevealSets
    let p: Vec<&str> = text.split(':').collect();
    let game_string = p[0];
    let reveal_sets = p[1];

    // parse game id out of game_string
    let game_id: u32 = game_string.trim().split("Game ").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    let mut game = Game {
        id: game_id, // Hardcoded ID for this example
        reveal_sets: Vec::new(),
    };

    // parse each reveal set
    for set in reveal_sets.split(';') {
        // TODO: no need for this to be mutable, just construct later
        let mut reveal_set = RevealSet {
            red: 0,
            blue: 0,
            green: 0,
        };

        for value_str in set.split(',') {
            let r: Vec<&str> = value_str.trim().split(' ').collect();
            let count: u32 = r[0].parse().unwrap();
            let color = r[1];
            match color.trim() {
                "green" => reveal_set.green = count,
                "red" => reveal_set.red = count,
                "blue" => reveal_set.blue = count,
                _ => panic!("Invalid color: {}", color),
            }
        }

        game.reveal_sets.push(reveal_set);
    }

    game
}
