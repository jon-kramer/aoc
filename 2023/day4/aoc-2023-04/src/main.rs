use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_reader = BufReader::new(File::open("../input").unwrap());
    // let output = part1(BufReader::new(stdin().lock()));
    // let output = part1(file_reader);
    let output = part2(file_reader);
    println!("Final count = {}", output);
}

fn part1<R>(reader: R) -> u32
where
    R: BufRead,
{
    reader
        .lines()
        .map(|line| {
            let points = get_points_for_line(&line.unwrap());
            println!("Line points = {}", points);
            points
        })
        .sum()
}

fn get_points_for_line(line: &str) -> u32 {
    let card = line.trim().split(": ");
    let mut card_nums = card.last().unwrap().trim().split(" | ");
    let winning_nums = card_nums
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .filter(|num| num != &"");
    let my_nums = card_nums
        .next()
        .unwrap()
        .trim()
        .split(' ') // 11, 11, 11
        .filter(|num| num != &"");

    // compare winning nums with my nums

    my_nums.fold(0, |acc, my_num| {
        println!("{} {}", acc, my_num);
        if winning_nums
            .clone()
            .any(|winning_num| winning_num.trim() == my_num.trim())
        {
            if acc == 0 {
                1
            } else {
                acc * 2
            }
        } else {
            acc
        }
    })
}

#[derive(Debug)]
struct Card {
    id: u32,
    matches: u32,
}

impl Card {
    fn from_line(line: &str) -> Card {
        let mut card = line.trim().split(": ");

        let id = card
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut card_nums = card.last().unwrap().trim().split(" | ");
        let winning_nums = card_nums
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .filter(|num| num != &"");
        let my_nums = card_nums
            .next()
            .unwrap()
            .trim()
            .split(' ') // 11, 11, 11
            .filter(|num| num != &"");

        // compare winning nums with my nums

        let matches = my_nums.fold(0, |acc, my_num| {
            if winning_nums
                .clone()
                .any(|winning_num| winning_num.trim() == my_num.trim())
            {
                acc + 1
            } else {
                acc
            }
        });
        Card { id, matches }
    }
}

fn part2<R>(reader: R) -> u32
where
    R: BufRead,
{
    // First parse and process cards
    let cards: Vec<Card> = reader
        .lines()
        .map(|line| Card::from_line(&line.unwrap()))
        .collect();

    // make a new map of card ID to card count
    // let mut card_map: HashMap<u32, u32> = cards.iter().map(|card| (card.id, 1)).collect();
    let mut card_counts: Vec<u32> = cards.iter().map(|_c| 1).collect();

    // Now replicate the cards
    for card in cards.iter() {
        // let card_count = *card_map.get(&card.id).unwrap();
        let card_count = card_counts[(card.id - 1) as usize];
        println!("Card {:?} with count {}", card, card_count);
        for count in 1..=card.matches {
            let card_id_to_replicate = (card.id + count) as usize;
            // println!("Increment card id: {}", card_id_to_replicate);
            card_counts[card_id_to_replicate - 1] += card_count;
            // card_map
            //     .entry(card_id_to_replicate)
            //     .and_modify(|count| *count += 1);
        }
    }

    // card_map.iter().map(|e| e.1).sum()
    card_counts.iter().sum()
}

#[cfg(test)]
mod test {

    use crate::{part1, part2};
    use pretty_assertions::assert_eq;
    use std::io::{BufReader, Cursor};

    #[test]
    fn part1_success() {
        // expecting 1, 4, and 0 points respectively, for a total of 5
        let test_input = "Card   1: 69 61 27 | 69 0 5
Card   2:  5 75 37 | 5 75 37 4
Card   3:  1  2  3 |  4  5  6";

        let result = part1(BufReader::new(Cursor::new(test_input.as_bytes())));
        assert_eq!(result, 5)
    }

    #[test]
    fn part2_success() {
        // expecting 1, 4, and 0 points respectively, for a total of 5
        let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part2(BufReader::new(Cursor::new(test_input.as_bytes())));
        assert_eq!(result, 30)
    }
}
