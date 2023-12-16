use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
struct Gear {
    row: usize,
    col: usize,
    parts: usize,
    ratio: usize,
}

impl Gear {}

#[derive(Clone)]
struct Grid {
    rows: Vec<Vec<char>>,
    gears: Vec<Gear>,
}

impl Grid {
    fn size(&self) -> usize {
        self.rows.clone().len()
    }
    fn build_from_input(path: &str) -> Self {
        println!("cwd: {}", env::current_dir().unwrap().display());
        let input = std::fs::read_to_string(path).unwrap();

        Grid {
            rows: input
                .lines()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
            gears: Vec::new(),
        }
    }

    fn build_gear_map(&self) -> Vec<Gear> {
        let mut gears = Vec::new();

        self.rows.iter().enumerate().for_each(|(row_i, row)| {
            row.iter().enumerate().for_each(|(col_i, col)| {
                if col.to_owned() == '*' {
                    let parts = self.clone().get_nearby_parts(row_i, col_i);
                    let gear = Gear {
                        row: row_i,
                        col: col_i,
                        parts: parts.len(),
                        ratio: if parts.len() == 2 {
                            parts.iter().fold(1, |acc, p| acc * p.digits)
                        } else {
                            0
                        },
                    };
                    gears.push(gear);
                }
            });
        });
        gears
    }

    fn get_nearby_parts(self, row: usize, col: usize) -> Vec<Part> {
        let mut parts: HashMap<String, Part> = HashMap::new();
        let self_len = self.size();

        for c in (std::cmp::max(
            0,
            usize::try_from(i32::abs(i32::try_from(col).unwrap() - 1)).unwrap(),
        ))..(std::cmp::min(row + 2, self.rows[0].len()))
        {
            for r in (std::cmp::max(
                0,
                usize::try_from(i32::abs(i32::try_from(row).unwrap() - 1)).unwrap(),
            ))..(std::cmp::min(row + 2, self_len))
            {
                if r == row && c == col {
                    continue;
                } else {
                    if self.rows[r][c].is_digit(10) {
                        let part = self.find_part_at(r, c);
                        parts.insert(format!("{:?}_{:?}", part.row, part.start_col), part);
                    }
                }
            }
        }

        parts.values().map(|v| v.to_owned()).collect()
    }

    fn get_parts(self) -> Vec<Part> {
        let mut parts = Vec::new();
        let mut current_digits: String = String::from("");

        self.rows.iter().enumerate().for_each(|(row_i, row)| {
            row.iter().enumerate().for_each(|(col_i, col)| {
                if col.is_digit(10) {
                    current_digits.push(col.to_owned())
                } else {
                    if current_digits.len() > 0 {
                        let mut part = Part {
                            row: row_i,
                            start_col: col_i - current_digits.len(),
                            end_col: col_i - 1,
                            digits: current_digits.parse().unwrap(),
                            neigbors: Vec::new(),
                        };

                        part.add_neigbors(&self);
                        parts.push(part);
                        current_digits.drain(..);
                    }
                }
            });

            if current_digits.len() > 0 {
                let mut part = Part {
                    row: row_i,
                    start_col: row.len() - current_digits.len(),
                    end_col: row.len() - 1,
                    digits: current_digits.parse().unwrap(),
                    neigbors: Vec::new(),
                };

                part.add_neigbors(&self);
                parts.push(part);
                current_digits.drain(..);
            }
        });

        parts
    }

    fn find_part_at(&self, row: usize, col: usize) -> Part {
        let mut l_bound = 0;
        let mut r_bound = self.rows[row].len();

        for i in (0..col).rev() {
            if !self.rows[row][i].is_digit(10) {
                l_bound = i + 1;
                break;
            }
        }

        for i in col..self.rows[row].len() {
            if !self.rows[row][i].is_digit(10) {
                r_bound = i - 1;
                break;
            }
        }

        println!(
            "finding part: {:?} - {:?}\n{:?}",
            l_bound, r_bound, self.rows[row]
        );

        let digits = (self.rows[row]
            [l_bound..std::cmp::min(self.rows[row].len() - 1, r_bound + 1)])
            .iter()
            .collect::<String>()
            .parse();

        Part {
            row,
            start_col: l_bound,
            end_col: r_bound,
            digits: if digits.is_ok() { digits.unwrap() } else { col },
            neigbors: Vec::new(),
        }
    }

    fn get_part_locations(self) -> Vec<Vec<char>> {
        let mut part_locations: Vec<Vec<char>> = Vec::new();

        let mut last_part: Option<usize> = None;
        let part_chars: Vec<Vec<char>> = vec![vec!['A', 'B'], vec!['M', 'N'], vec!['X', 'Y']];
        let mut part_index: usize = 0;

        self.rows.iter().enumerate().for_each(|(row_i, row)| {
            let mut parts_row: Vec<char> = Vec::new();
            last_part = None;
            part_index = 0;
            row.iter().enumerate().for_each(|(col_i, col)| {
                if col.is_digit(10) {
                    parts_row.push(part_chars[row_i % 3][part_index]);
                    last_part = Some(col_i);
                } else {
                    if last_part.is_some() && last_part.unwrap() + 1 == col_i {
                        part_index = (part_index + 1) % 2;
                    }
                    parts_row.push(col.to_owned());
                }
            });
            part_locations.push(parts_row);
        });
        part_locations
    }
}

#[derive(Debug, Clone)]
struct Part {
    row: usize,
    start_col: usize,
    end_col: usize,
    digits: usize,
    neigbors: Vec<char>,
}

impl Part {
    fn add_neigbors(&mut self, grid: &Grid) {
        let mut neigbors: Vec<char> = Vec::new();

        for c in (std::cmp::max(
            0,
            usize::try_from(i32::abs((i32::try_from(self.start_col).unwrap() - 1))).unwrap(),
        ))..(std::cmp::min(self.end_col + 2, grid.rows[0].len()))
        {
            for r in (std::cmp::max(
                0,
                usize::try_from(i32::abs((i32::try_from(self.row).unwrap() - 1))).unwrap(),
            ))..(std::cmp::min(self.row + 2, grid.rows.len()))
            {
                if r == self.row && c >= self.start_col && c <= self.end_col {
                    continue;
                } else {
                    if grid.rows[r][c] != '.' {
                        neigbors.push(grid.rows[r][c].clone());
                    }
                }
            }
        }
        self.neigbors = neigbors;
    }

    fn is_next_to_symbol(&self) -> bool {
        self.neigbors
            .iter()
            .any(|n| !n.is_digit(10) && n != &".".chars().nth(0).unwrap())
    }
}

fn main() {
    let grid_main: Grid = Grid::build_from_input("./input.txt");
    let mut grid_test: Grid = Grid::build_from_input("./test_input.txt");
    let gears = grid_main.build_gear_map();

    println!(
        "{:?}",
        gears
            .iter()
            .filter(|g| g.parts == 2)
            .fold(1, |acc, g| acc + g.ratio)
    );

    // let parts_test: Vec<Part> = grid_test.get_parts();
    // let part_locations_test = grid_test.get_part_locations();

    // let sum_main = parts_main
    //     .iter()
    //     .filter(|p| p.is_next_to_symbol())
    //     .fold(0, |acc, p| acc + p.digits);
    //
    // let sum_test = parts_test
    //     .iter()
    //     .filter(|p| p.is_next_to_symbol())
    //     .fold(0, |acc, p| acc + p.digits);
    //
    // println!("{:?}", sum_main);
    // println!("{:?}", sum_test);
}
