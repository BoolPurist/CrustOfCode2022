use core::iter::repeat;
use core::num::IntErrorKind;
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Digit(u8);
type TreeGrid = Vec<Vec<Digit>>;
pub fn get_number_of_visible(input: &str) -> usize {
    let input_parsed = parse_input(input);

    let height = input_parsed.len();
    let width = input_parsed
        .iter()
        .nth(1)
        .expect("No single row in grid")
        .len();
    const FACTOR: usize = 2;
    let visible_from_edges = (FACTOR * height) + (FACTOR * width) - 4usize;
    let without_edges = get_view_no_edges(&input_parsed);
    let o = get_visible_from_row(&vec![
        Digit(2),
        Digit(2),
        Digit(4),
        Digit(2),
        Digit(5),
        Digit(4),
    ]);
    dbg!(o);

    let number_visible = visible_from_edges;

    number_visible
}
#[derive(Debug)]
struct DigitMaximum(usize, Digit);
fn get_visible_from_row(row: &Vec<Digit>) -> Vec<bool> {
    let length = row.len();
    let mut output: Vec<bool> = repeat(false).take(length).collect();

    let mut current_length = length;
    let mut current_start: usize = 0;
    let mut current_left_max = DigitMaximum(0, Digit::default());
    let mut two_max_equal = false;
    let mut current_right_max = DigitMaximum(0, Digit::default());
    for i in (&row[current_start..current_length]).iter().enumerate() {
        let (index, current_digit) = i;

        if *current_digit > current_right_max.1 {
            current_left_max = current_right_max;
            two_max_equal = false;
            current_right_max = DigitMaximum(index, current_digit.clone());
        } else if *current_digit == current_right_max.1 {
            if two_max_equal {
                current_right_max = DigitMaximum(index, current_digit.clone());
            } else {
                current_left_max = current_right_max;
                two_max_equal = true;
                current_right_max = DigitMaximum(index, current_digit.clone());
            }
        }
    }

    output[current_left_max.0] = true;
    output[current_right_max.0] = true;
    output
}

fn get_view_no_edges<'a>(with_edges: &'a TreeGrid) -> TreeGrid {
    let to_take = with_edges.len() - 2usize;
    with_edges
        .iter()
        .skip(1)
        .take(to_take)
        .map(|chars| {
            let to_take_from_chars = chars.len() - 2usize;
            chars
                .iter()
                .skip(1)
                .take(to_take_from_chars)
                .cloned()
                .collect::<Vec<Digit>>()
        })
        .collect()
}

fn parse_input(input: &str) -> TreeGrid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| Digit::from(char).expect("Could not parse char into digit"))
                .collect()
        })
        .collect()
}

impl Digit {
    fn from(digit: char) -> Result<Self, IntErrorKind> {
        let number: u32 = digit.to_digit(10).ok_or(IntErrorKind::InvalidDigit)?;
        let shrinked = u8::try_from(number).map_err(|_| IntErrorKind::PosOverflow)?;

        Ok(Self(shrinked))
    }

    fn value(&self) -> u8 {
        self.0
    }

    fn is_not_blocked_by(&self, other: &Self) -> bool {
        other.0 < self.0
    }
}
