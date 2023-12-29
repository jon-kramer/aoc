use std::{
    collections::{hash_map, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Error},
    ops::Range,
};

fn main() {
    let file_reader = BufReader::new(File::open("../input").unwrap());
    // let output = part1(BufReader::new(stdin().lock()));
    // let output = part1(file_reader);
    let output = part1(file_reader);
    println!("Output: {}", output);
    // ExitCode::FAILURE
}

/*
* read the file lines in paragraphs
*
* 1. get seed lines and parse
* 2. skip empty line
* 3. read range map ("seed-to-soil")
* 4. (continue reading range maps)
* 5. use mappings to reach locations for each seed
* 6. return min seed location
*
*/

fn part1<R>(reader: R) -> u64
where
    R: BufRead,
{
    let mut iterator = reader.lines();
    let seeds_string = iterator.next().unwrap().unwrap();
    let seed_ids: Vec<u64> = seeds_string
        .split("seeds: ")
        .last()
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    println!("{:?}", seed_ids);
    iterator.next(); // ignore empty line

    let mapping_chain: Vec<_> = (0..7).map(|_c| parse_next_map(&mut iterator)).collect();

    seed_ids
        .iter()
        .map(|seed_id| {
            println!("Seed ID: {}", seed_id);
            let mut curr_val = *seed_id;
            for mapping in &mapping_chain {
                // println!("{:#?}", mapping);
                curr_val = evaluate_mapping(curr_val, mapping);
                println!("{}", curr_val);
            }
            curr_val
        })
        .min()
        .unwrap()
}

fn parse_next_map<I: Iterator<Item = Result<String, Error>>>(
    mut iterator: I,
) -> HashSet<MappingRange> {
    // Remove first map line
    let map_line = iterator.next().unwrap().unwrap();
    println!("{}", map_line);

    iterator
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| MappingRange::from_line(&line))
        .collect()
}

fn evaluate_mapping(value: u64, mapping: &HashSet<MappingRange>) -> u64 {
    let mapping_match = mapping
        .iter()
        .find(|m| m.get_source_range().contains(&value));

    match mapping_match {
        Some(m) => (value - m.source_range_start) + m.dest_range_start,
        None => value,
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct MappingRange {
    dest_range_start: u64,
    source_range_start: u64,
    range_len: u64,
}

impl MappingRange {
    fn from_line(line: &str) -> MappingRange {
        let mut nums = line.split(' ').map(|s| s.parse::<u64>().unwrap());
        MappingRange {
            dest_range_start: nums.next().unwrap(),
            source_range_start: nums.next().unwrap(),
            range_len: nums.next().unwrap(),
        }
    }
    fn get_dest_range(&self) -> Range<u64> {
        self.dest_range_start..(self.dest_range_start + self.range_len)
    }
    fn get_source_range(&self) -> Range<u64> {
        self.source_range_start..(self.source_range_start + self.range_len)
    }
}

/*
* Treat ranges as a graph?
* DFS the different parts of the range mapping
*
*
* RangeSet
* diff [) [)
*
*/
fn part2<R>(reader: R) -> u64
where
    R: BufRead,
{
    5
}

#[cfg(test)]
mod test {

    use std::io::{BufReader, Cursor};

    use pretty_assertions::assert_eq;

    use crate::part1;

    #[test]
    fn part1_success() {
        // expecting 1, 4, and 0 points respectively, for a total of 5
        let test_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result = part1(BufReader::new(Cursor::new(test_input.as_bytes())));
        assert_eq!(result, 35)
    }
}
