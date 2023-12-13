use std::{
    io::stdin,
    ops::{Range, RangeInclusive},
};

use grid::{construct_grid_from_input, LinearGrid};
use regex::Regex;

mod grid;

fn main() {
    part2();
}

/*
*
* Assume grid with fixed width?
*  - Need nested arrays or can use x + n*y to access linear grid.
*  i.e. 10x10 and want to access (3,4), this would be 3 + (10 * 4)
*
* Care about memory?
*  - Load all into memory OR store a ring buffer of 3 lines of context
*
* Simple solution:
*  1. Load all lines into a single Vec<u8> (aka an ascii String, but this simplifies indexing).
*     Find grid width (length of any line),
*     and track grid height (from number of lines) - also height = Vec.len / width...
*  2. iterate through all grid positions, checking all adjacent characters of any .
*     must keep boundaries in mind.. do not exceed grid width or height
*
*/

/*
* Lets try this different in part2:
* . for each line
* . find all numbers 2. find all symbols (or for part 2 just *)
* . for each symbol find adjacent numbers
* . take the * with exactly 2 adjacent numbers, calculate gear ratio
* . get final sum
*/
fn part2() {
    // TODO: move to different function

    let mut row_numbers: Vec<Vec<RowNumber>> = vec![];
    let mut row_parts: Vec<Vec<RowPart>> = vec![];
    let mut ratio_sum = 0;

    for line in stdin().lines() {
        let line = line.unwrap();
        // Remove any newline chars, also lets assume ascii for simpler indexing
        let trimmed = line.trim();
        row_numbers.push(find_row_nums(trimmed));
        row_parts.push(find_row_parts(trimmed));
    }

    for (y, parts) in row_parts.iter().enumerate() {
        let y_range_lower = if y == 0 { y } else { y - 1 };
        let y_range_upper = if y == row_parts.len() - 1 { y } else { y + 1 };

        for part in parts {
            // As an optimization we only need to look at the numbers above, at, and below the part
            // row

            let part_x_lower = if part.x == 0 { part.x } else { part.x - 1 };
            let part_x_upper = if part.x == 139 { part.x } else { part.x + 1 };
            // println!("{} to {}", &part_x_lower, &part_x_upper);

            let adj_nums: Vec<&RowNumber> = (y_range_lower..=y_range_upper)
                .flat_map(|num_y| {
                    row_numbers[num_y].iter().filter(|n| -> bool {
                        n.x_range.start() <= &part_x_upper && n.x_range.end() >= &part_x_lower
                    })
                })
                .collect();

            // println!("{:?}", adj_nums);

            if adj_nums.len() == 2 {
                let ratio = adj_nums[0].value * adj_nums[1].value;
                ratio_sum += ratio;
            }
        }
    }

    println!("{}", ratio_sum);
}

fn find_row_parts(row: &str) -> Vec<RowPart> {
    let mut row_parts: Vec<RowPart> = vec![];
    for (i, c) in row.chars().enumerate() {
        // if !c.is_numeric() && c != '.' {
        if c == '*' {
            row_parts.push(RowPart { x: i as u32, c: c });
        }
    }
    row_parts
}

fn find_row_nums(row: &str) -> Vec<RowNumber> {
    let re = Regex::new(r"\d+").unwrap();
    let mut row_nums: Vec<RowNumber> = vec![];

    for m in re.find_iter(row) {
        // let match_x_range = m.start() as u32..(m.end() + 1) as u32;
        let match_x_range = m.start() as u32..=(m.end() - 1) as u32;
        let match_value = m.as_str().parse::<u32>().unwrap();
        row_nums.push(RowNumber {
            x_range: match_x_range,
            value: match_value,
        })
    }
    // println!("{:?}", row_nums);
    row_nums
}

#[derive(Debug)]
struct RowNumber {
    x_range: RangeInclusive<u32>,
    value: u32,
}
struct RowPart {
    x: u32,
    c: char,
}

struct NumberCursor {
    num: u32,
    next_place: u32,
    found_symbol: bool,
}

fn part1() {
    let g = construct_grid_from_input(140);

    // loop through the grid characters
    // we do this in reverse order to simplify number parsing.
    //
    // seek to the left
    // if we find a number,
    //   if we already have a curr_num
    //     add to it and adjust the num_place
    //     check for adjacent symbols
    // else // if we find a symbol or .
    //    if we have a curr_num AND found_symbol
    //
    // seek to the left find the rest of the number
    // this requires fine grain control of the iteration..
    // as we find adjacent digits, we should check for adjacent symbols
    // and track if at least one adjacent was found.

    let mut curr_num: Option<NumberCursor> = None;
    let mut part_nums: Vec<u32> = vec![];
    let mut part_sum: u32 = 0;

    for y in 0..g.height {
        println!("{} ==================", y);
        for x in (0..g.width).rev() {
            let value_at_coord = g.value_at_coord(x, y);
            let c = value_at_coord.unwrap() as char;
            if c.is_numeric() {
                let new_num = c.to_digit(10).unwrap();
                match curr_num {
                    Some(n) => {
                        // if we havent yet found the symbol for this number, attempt to find it
                        let mut found_symbol_for_num = n.found_symbol;
                        if !n.found_symbol {
                            found_symbol_for_num =
                                n.found_symbol || check_adjacents_for_symbol(x, y, &g);
                        }
                        curr_num = Some(NumberCursor {
                            num: n.num + (new_num * n.next_place),
                            next_place: n.next_place * 10,
                            found_symbol: found_symbol_for_num,
                        })
                    }
                    None => {
                        let has_adjacent_symbol = check_adjacents_for_symbol(x, y, &g);
                        curr_num = Some(NumberCursor {
                            num: new_num,
                            next_place: 10,
                            found_symbol: has_adjacent_symbol,
                        })
                    }
                }
            } else {
                // If we have a curr_num, then finding a non-numeric char terminates it
                // if the curr num has a found symbol, add it to our part list
                // finally reset curr_num to None
                //
                if let Some(n) = curr_num {
                    println!("{}", n.num);
                    if n.found_symbol {
                        part_nums.push(n.num);
                        println!("{}", n.num);
                        part_sum += n.num;
                    }
                    curr_num = None;
                }
            }
        }
        // TODO: meh duplicated code....
        //  perhaps should move the termination logic to inline with the loop?
        if let Some(n) = curr_num {
            println!("{}", n.num);
            if n.found_symbol {
                part_nums.push(n.num);
                part_sum += n.num;
            }
            curr_num = None;
        }
    }

    println!("{}", part_sum);
}

fn check_adjacents_for_symbol(x: usize, y: usize, g: &LinearGrid) -> bool {
    // Probably can just directly check adjacents, but this is a decent abstraction
    get_valid_adjacents(x, y, g).iter().any(|adj| {
        g.value_at_coord(adj.0, adj.1)
            .map_or(false, |c| !(c as char).is_numeric() && (c as char) != '.')
    })
}

fn get_valid_adjacents(x: usize, y: usize, g: &LinearGrid) -> Vec<(usize, usize)> {
    let mut adjacents = vec![];

    let up_valid = y > 0;
    let down_valid = y + 1 < g.height;
    let right_valid = x + 1 < g.width;
    let left_valid = x > 0;

    if up_valid && left_valid {
        adjacents.push((x - 1, y - 1));
    }
    if up_valid {
        adjacents.push((x, y - 1));
    }
    if up_valid && right_valid {
        adjacents.push((x + 1, y - 1));
    }
    if right_valid {
        adjacents.push((x + 1, y));
    }
    if down_valid && right_valid {
        adjacents.push((x + 1, y + 1));
    }
    if down_valid {
        adjacents.push((x, y + 1));
    }
    if down_valid && left_valid {
        adjacents.push((x - 1, y + 1));
    }
    if left_valid {
        adjacents.push((x - 1, y));
    }
    adjacents
}

fn _check_adjacent_old(x: usize, y: usize, g: LinearGrid) -> Vec<u32> {
    // attempt to parse_number in each direction
    // return a list of any numbers that were found
    //
    // there is a complication... we can have multiple adjacents point
    // at the same number...
    //  12345
    //  ..*..
    // in this case NW, N, and NE all point to 12345
    //
    // alternatively, we could have
    //  123.123
    //  ...*...
    // in this case NW and NE each point to unique instances of 123
    //
    // The problem is that we can not distinguish between these cases.
    Vec::new()
}
