use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Part {
    row: usize,
    start_col: usize,
    end_col: usize,
    digits: usize,
}

fn parse_text(str_input: &str) -> (Vec<Part>, Vec<(usize, usize)>) {
    let mut current_digits: String = String::from("");
    let mut parts: Vec<Part> = Vec::new();
    let mut symbols: Vec<(usize, usize)> = Vec::new();

    str_input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, ch)| {
            if ch.is_digit(10) {
                current_digits.push(ch)
            } else {
                if ch != '.' {
                    symbols.push((row, col));
                }
                if !current_digits.is_empty() {
                    parts.push(Part {
                        row,
                        start_col: col - current_digits.len(),
                        end_col: col - 1,
                        digits: current_digits.parse().unwrap(),
                    });
                    current_digits.drain(..);
                }
            }
        });

        if !current_digits.is_empty() {
            let col = line.len();
            println!("{row}, {col}, {:?}", current_digits);
            parts.push(Part {
                row,
                start_col: col - current_digits.len(),
                end_col: col - 1,
                digits: current_digits.parse().unwrap(),
            });
            current_digits.drain(..);
        }
    });

    (parts, symbols)
}

fn get_parts(str_input: &str) -> Vec<Part> {
    let mut current_digits = String::from("");
    let mut parts: Vec<Part> = Vec::new();

    let mut prev_symbols: HashSet<usize> = HashSet::new();
    let mut curr_symbols: HashSet<usize> = HashSet::new();

    let mut prev_parts: HashMap<usize, Vec<Part>> = HashMap::new();
    let mut curr_parts: HashMap<usize, Vec<Part>> = HashMap::new();

    let mut last_symbol_seen: Option<usize> = None;
    let mut last_part_seen: Option<Part> = None;

    str_input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, ch)| {
            // If a digit is found...
            if ch.is_digit(10) {
                // Build the digits that will eventually be represented as a Part
                current_digits.push(ch)
            } else {
                // If a part has just finished building
                if !current_digits.is_empty() {
                    let part = Part {
                        row,
                        start_col: col - current_digits.len(),
                        end_col: col - 1,
                        digits: current_digits.parse().unwrap(),
                    };

                    last_part_seen = Some(part.clone());

                    if part.start_col > 0 {
                        curr_parts.entry(part.start_col - 1).or_insert(Vec::new());
                        curr_parts
                            .entry(part.start_col - 1)
                            .and_modify(|arr| arr.push(part.clone()));
                    }
                    for i in part.start_col..part.end_col + 2 {
                        curr_parts.entry(i).or_insert(Vec::new());
                        curr_parts.entry(i).and_modify(|arr| arr.push(part.clone()));
                    }
                    if last_symbol_seen.is_some()
                        && part.start_col > 0
                        && part.start_col == (last_symbol_seen.unwrap() + 1)
                    {
                        parts.push(part.clone())
                    } else {
                        for i in (col - current_digits.len())..col {
                            let idx = i.clone();
                            if prev_symbols.contains(&idx) {
                                parts.push(part.clone());
                                break;
                            }
                        }
                    }

                    current_digits.drain(..);
                }

                // If a symbol is found...
                if ch != '.' {
                    // Add the current index to the current Vector for symbol indeces
                    if col > 0 {
                        curr_symbols.insert(col - 1);
                    }
                    curr_symbols.insert(col);
                    curr_symbols.insert(col + 1);

                    // Set the index for the last symbol seen
                    last_symbol_seen = Some(col);

                    // If there is a part left of col
                    if let Some(last_part) = last_part_seen.clone() {
                        if last_part.end_col + 1 == col {
                            parts.push(last_part.clone());
                        }
                    }

                    // list of Parts
                    if prev_parts.contains_key(&col) {
                        for p in &prev_parts[&col] {
                            parts.push(p.to_owned());
                        }
                    }
                }
            }
        });

        if !current_digits.is_empty() {
            let col = line.len();
            let part = Part {
                row,
                start_col: col - current_digits.len(),
                end_col: col - 1,
                digits: current_digits.parse().unwrap(),
            };
            last_part_seen = Some(part.clone());

            if part.start_col > 0 {
                curr_parts.entry(part.start_col - 1).or_insert(Vec::new());
                curr_parts
                    .entry(part.start_col - 1)
                    .and_modify(|arr| arr.push(part.clone()));
            }

            for i in part.start_col..part.end_col + 2 {
                curr_parts.entry(i).or_insert(Vec::new());
                curr_parts.entry(i).and_modify(|arr| arr.push(part.clone()));
            }

            for i in part.start_col..part.end_col + 1 {
                println!("adding part ref");
                curr_parts.entry(i).or_insert(Vec::new());
                curr_parts.entry(i).and_modify(|arr| arr.push(part.clone()));
            }

            if let Some(symbol_idx) = last_symbol_seen {
                if symbol_idx == part.start_col - 1 {
                    parts.push(part.clone())
                }
            } else {
                for i in part.start_col..part.end_col {
                    let idx = i.clone();
                    if prev_symbols.contains(&idx) {
                        parts.push(part.clone());
                        break;
                    }
                }
            }
            current_digits.drain(..);
        }

        last_symbol_seen = None;
        prev_symbols = curr_symbols.clone();
        curr_symbols.drain();

        last_part_seen = None;
        prev_parts = curr_parts.clone();
        curr_parts.drain();
    });

    parts
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let parts = get_parts(&input);
    let relevant_parts: Vec<usize> = parts.iter().map(|p| p.digits).collect();
    println!("relevant_parts: {:?}", relevant_parts);

    let sum = parts.iter().fold(0, |acc, p| acc + p.digits);

    println!("{:?}", sum);
}
