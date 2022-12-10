use core::iter::repeat;
use core::num::IntErrorKind;

use crate::sequences;

type TreeGrid = Vec<Vec<Digit>>;
type TreeVisibility = Vec<Vec<bool>>;
type TreeSenicScores = Vec<Vec<usize>>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Digit(i8);

pub fn get_number_of_visible(input: &str) -> usize {
    let input_parsed = parse_input(input);
    let visibility = inpect_grid_for_visible(&input_parsed);

    visibility.iter().fold(usize::default(), |akk, row| {
        akk + row.iter().fold(
            usize::default(),
            |akk, cell| if *cell { akk + 1 } else { akk },
        )
    })
}

pub fn get_max_scenic_score(input: &str) -> usize {
    let grid = parse_input(input);
    let scores = create_grid_with_scenic_score(&grid);

    scores
        .iter()
        .map(|row| *row.iter().max().expect("No maximum for a row"))
        .max()
        .expect("No maximum at all")
}

fn create_grid_with_scenic_score(grid: &TreeGrid) -> TreeSenicScores {
    let height = grid.len();
    let width = grid[0].len();

    let mut scenic_scores: TreeSenicScores = sequences::create_grid_with_value(height, width, &1);

    fill_grid_score_brute_force(grid, &mut scenic_scores);

    scenic_scores
}

fn fill_grid_score_brute_force(grid: &TreeGrid, scores: &mut TreeSenicScores) {
    let height = grid.len();
    let width = grid[0].len();
    // zero out first row
    for x in 0..width {
        scores[0][x] = 0;
    }
    // zero out last row
    let last_row = scores.last_mut().unwrap();
    for x in 0..width {
        last_row[x] = 0;
    }

    for y in 1..(height - 1) {
        *scores[y].first_mut().unwrap() = 0;
        *scores[y].last_mut().unwrap() = 0;

        let before_end = width - 1;
        for x in 1..before_end {
            let current_element = grid[y][x];

            scores[y][x] *=
                get_view_dist_from_dim(sequences::grid_traverse_up(y), &current_element, |next| {
                    grid[next][x]
                });

            scores[y][x] *= get_view_dist_from_dim(
                sequences::grid_traverse_down(y, height),
                &current_element,
                |next| grid[next][x],
            );

            scores[y][x] *= get_view_dist_from_dim(
                sequences::grid_traverse_left(x),
                &current_element,
                |next| grid[y][next],
            );

            scores[y][x] *= get_view_dist_from_dim(
                sequences::grid_traverse_right(x, width),
                &current_element,
                |next| grid[y][next],
            );
        }
    }

    fn get_view_dist_from_dim<F>(
        dim: impl Iterator<Item = usize>,
        current_element: &Digit,
        indexer: F,
    ) -> usize
    where
        F: Fn(usize) -> Digit,
    {
        let mut counter = 0;
        for next in dim {
            counter += 1;
            let next_element = indexer(next);
            if *current_element <= next_element {
                break;
            }
        }

        counter
    }
}

#[allow(dead_code)]
fn draw_visibility(draw_from: &TreeVisibility) -> String {
    let chars: Vec<String> = draw_from
        .iter()
        .map(|row| {
            let mut row_buffer = String::new();

            for visible in row {
                let next_char = if *visible { 'X' } else { '*' };
                row_buffer.push(next_char);
            }

            row_buffer
        })
        .collect();

    chars.join("\n")
}

fn inpect_grid_for_visible(grid: &TreeGrid) -> TreeVisibility {
    let height = grid.len();
    let width = grid[0].len();
    let mut visibility = sequences::create_grid_with_default(height, width);

    for next_row in 0..height {
        let next_seq = get_iter_rows(next_row, width);
        inspect_sequence_for_visble(grid, &mut visibility, next_seq);
    }

    for next_column in 0..width {
        let next_seq = get_iter_column(next_column, height);
        inspect_sequence_for_visble(grid, &mut visibility, next_seq);
    }

    visibility
}

fn get_iter_rows(row_index: usize, width: usize) -> Vec<(usize, usize)> {
    repeat(row_index)
        .take(width)
        .enumerate()
        .map(|x_y| (x_y.1, x_y.0))
        .collect()
}

fn get_iter_column(column_index: usize, height: usize) -> Vec<(usize, usize)> {
    repeat(column_index).take(height).enumerate().collect()
}

fn inspect_sequence_for_visble(
    grid: &Vec<Vec<Digit>>,
    visible: &mut Vec<Vec<bool>>,
    mut sequence: Vec<(usize, usize)>,
) {
    // left
    let mut last_max = Digit::default();

    for (y, x) in sequence.iter() {
        let current_digit = grid[*y][*x];
        if current_digit > last_max {
            last_max = current_digit.clone();
            visible[*y][*x] = true;
        }
    }

    // right
    last_max = Digit::default();
    sequence.reverse();

    for (y, x) in sequence {
        let current_digit = grid[y][x];
        if current_digit > last_max {
            last_max = current_digit.clone();
            visible[y][x] = true;
        }
    }
}

#[allow(dead_code)]
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
        let shrinked = i8::try_from(number).map_err(|_| IntErrorKind::PosOverflow)?;

        Ok(Self(shrinked))
    }
}

impl Default for Digit {
    fn default() -> Self {
        Self(-1)
    }
}
