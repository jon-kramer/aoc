mod interval;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    ops::Range,
};

use crate::interval::Interval;

fn main() {
    let file_reader = BufReader::new(File::open("../input").unwrap());
    // let output = part1(BufReader::new(stdin().lock()));
    // let output = part1(file_reader);
    let output = part2(file_reader);
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

fn parse_next_map<I: Iterator<Item = Result<String, Error>>>(mut iterator: I) -> Vec<MappingRange> {
    // Remove first map line
    let map_line = iterator.next().unwrap().unwrap();
    println!("PARSING {}", map_line);

    let mut map = iterator
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| MappingRange::from_line(&line))
        .collect::<Vec<MappingRange>>();
    map.sort_by(|m1, m2| m1.source_range_start.cmp(&m2.source_range_start));
    map
}

fn evaluate_mapping(value: u64, mapping: &Vec<MappingRange>) -> u64 {
    let mapping_match = mapping
        .iter()
        .find(|m| m.get_source_range().contains(&value));

    match mapping_match {
        Some(m) => (value - m.source_range_start) + m.dest_range_start,
        None => value,
    }
}

fn offset_mapping_range(value: u64, m: &MappingRange) -> u64 {
    value - m.source_range_start + m.dest_range_start
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
* starting interval(s) ... then find its intersection with the next mapping
*
* (5,10) intersect with (3,6) (9,11) to output (3,6)' (6,9) (9,10)'
*
* check each interval against each other interval... output a tuple with an interval
* that intersects and a part that doesn't.
*
*
*
*/
fn part2<R>(reader: R) -> u64
where
    R: BufRead,
{
    let mut iterator = reader.lines();
    let seed_lines = iterator.next().unwrap().unwrap();
    let seed_strings: Vec<u64> = seed_lines
        .split("seeds: ")
        .last()
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let seed_id_intervals: Vec<Interval> = seed_strings
        .chunks(2)
        .map(|w| Interval(w[0], w[1] + w[0]))
        .collect();

    println!("{:?}", seed_id_intervals);
    iterator.next(); // ignore empty line

    let mapping_chain: Vec<_> = (0..7).map(|_c| parse_next_map(&mut iterator)).collect();

    let mut curr_intervals = seed_id_intervals;
    println!("Starting seed intervals: {:?}", curr_intervals);
    for mapping in &mapping_chain {
        // println!("MAPPING_CHAIN =================");
        curr_intervals = evaluate_mapping_for_intervals(&curr_intervals, &mapping);
        // println!("Intervals post-mapping: {:?}", curr_intervals);
    }
    curr_intervals.iter().map(|i| i.0).min().unwrap()
}

fn evaluate_mapping_for_intervals(
    interval_values: &[Interval],
    mapping: &[MappingRange],
) -> Vec<Interval> {
    // Assuming the mapping ranges are ordered by increasing source start range,
    // we can safely iterate through them without having to go back and
    // reevaluate interval diffs generated along the way.

    let mut mapped = Vec::new();
    let mut intervals_to_process = interval_values.to_vec();
    for m in mapping {
        // println!("PROCESS INTERVALS: {:?}", intervals_to_process);
        // println!("MAPPING RANGE: {:?}", m);
        let mut next_intervals = Vec::new();
        for interval_value in intervals_to_process {
            let intersection = interval_value.intersect(&Interval(
                m.source_range_start,
                m.source_range_start + m.range_len,
            ));
            // println!("{:?}", intersection);
            match intersection {
                Some(intersect) => {
                    mapped.push(Interval(
                        offset_mapping_range(intersect.0, m),
                        offset_mapping_range(intersect.1, m),
                    ));
                    next_intervals.append(&mut interval_value.diff(&intersect));
                }
                None => {
                    next_intervals.push(interval_value.clone());
                }
            }
        }
        intervals_to_process = next_intervals
    }
    mapped.append(&mut intervals_to_process);
    mapped
}

#[cfg(test)]
mod test {

    use std::io::{BufReader, Cursor};

    use pretty_assertions::assert_eq;

    use crate::{part1, part2};

    const TEST_INPUT: &str = "seeds: 79 14 55 13

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

    #[test]
    fn part1_success() {
        let result = part1(BufReader::new(Cursor::new(TEST_INPUT.as_bytes())));
        assert_eq!(result, 35)
    }

    #[test]
    fn part2_success() {
        let result = part2(BufReader::new(Cursor::new(TEST_INPUT.as_bytes())));
        assert_eq!(result, 46)
    }
}
