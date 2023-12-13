#[derive(Debug)]
struct Part {
    row: usize,
    start_col: usize,
    end_col: usize,
    digits: usize,
}

fn compare_lines(parts: Vec<Part>, symbols: Vec<(usize, usize)>):


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

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let (parts, symbols) = parse_text(&input);
    // println!("{:?}, {:?}", parts, symbols);
}
