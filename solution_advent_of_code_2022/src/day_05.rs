use crate::parsing;
use core::str::FromStr;
use std::collections::VecDeque;
type CrateCells = Vec<Option<char>>;
type CrateToFillIn = Vec<CrateCells>;

pub fn get_tops_stack_9000(input: &str) -> String {
    let mut crane = parse_input(input);
    crane.process_instructions_by_9000();

    convert_crane_to_output(&crane)
}

pub fn get_tops_stack_9001(input: &str) -> String {
    let mut crane = parse_input(input);
    crane.process_instructions_by_9001();

    convert_crane_to_output(&crane)
}

fn convert_crane_to_output(crane: &CraneInProgress) -> String {
    let chars = crane.get_tops();
    let mut output = String::with_capacity(chars.len());
    chars.into_iter().for_each(|to_push| output.push(to_push));

    output
}

#[derive(Debug)]
struct Instruction {
    movement: u32,
    start: u32,
    dest: u32,
}

impl Instruction {
    fn get_start_index(&self) -> usize {
        (self.start - 1) as usize
    }
    fn get_dest_index(&self) -> usize {
        (self.dest - 1) as usize
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seq = parsing::get_seq_from_regex(r"move (\d+) from (\d+) to (\d+)", s, 3)
            .expect("Could not parse instruction");

        let mut numbers = seq
            .into_iter()
            .map(|to_parse| to_parse.parse::<u32>().expect("Could not parse to number"));

        match (numbers.next(), numbers.next(), numbers.next()) {
            (Some(movement), Some(start), Some(dest)) => Ok(Self {
                movement,
                start,
                dest,
            }),
            _ => panic!("Could not parse instruction"),
        }
    }
}
#[derive(Debug)]
struct CraneInProgress {
    crate_stacks: Vec<Vec<char>>,
    to_do: Vec<Instruction>,
}

impl Default for CraneInProgress {
    fn default() -> Self {
        Self {
            crate_stacks: Vec::new(),
            to_do: Vec::new(),
        }
    }
}

impl CraneInProgress {
    fn init(&mut self, number_of_stacks: usize) {
        for _ in 0..number_of_stacks {
            self.crate_stacks.push(Vec::new());
        }
    }

    fn fill_in_crates(&mut self, to_fill_in: CrateToFillIn) {
        for create_level in to_fill_in.into_iter() {
            for (index, cell) in create_level.into_iter().enumerate() {
                if let Some(letter) = cell {
                    self.crate_stacks[index].push(letter);
                }
            }
        }
    }

    fn process_instructions_by_9000(&mut self) {
        for next_to_do in &self.to_do {
            for _ in 0..(next_to_do.movement) {
                let popped = &self.crate_stacks[next_to_do.get_start_index()]
                    .pop()
                    .expect("No crate left to take");
                self.crate_stacks[next_to_do.get_dest_index()].push(*popped);
            }
        }
    }

    fn get_tops(&self) -> Vec<char> {
        self.crate_stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .copied()
            .collect()
    }

    fn process_instructions_by_9001(&mut self) {
        for next_to_do in &self.to_do {
            let mut to_move: VecDeque<char> = VecDeque::new();

            for _ in 0..(next_to_do.movement) {
                let popped = &self.crate_stacks[next_to_do.get_start_index()]
                    .pop()
                    .expect("No crate left to take");
                to_move.push_front(*popped);
            }

            self.crate_stacks[next_to_do.get_dest_index()].extend(to_move.into_iter());
        }
    }
}

const OFFSET_STACK_NUM_INST: usize = 2;
fn parse_input(input: &str) -> CraneInProgress {
    let mut crane = CraneInProgress::default();
    let lines: Vec<&str> = input.lines().collect();
    let to_bottom_crates = lines
        .iter()
        .position(|line| line.chars().any(|c| c.is_digit(10)))
        .expect("no stack number line found");

    let start_index_inst = to_bottom_crates + OFFSET_STACK_NUM_INST;
    extract_crates_from_input(&mut crane, &lines, to_bottom_crates);

    crane.to_do = lines
        .into_iter()
        .skip(start_index_inst)
        .map(|line| line.parse().unwrap())
        .collect();

    crane
}

fn extract_crates_from_input(
    crane: &mut CraneInProgress,
    lines: &Vec<&str>,
    to_bottom_crates: usize,
) {
    let crates: Vec<CrateCells> = lines
        .iter()
        .take(to_bottom_crates)
        .rev()
        .map(|line| get_crates_from_line(line))
        .collect();

    let number_creates_line = crates
        .iter()
        .take(1)
        .fold(usize::default(), |akk, line| akk + line.len());

    crane.init(number_creates_line);
    crane.fill_in_crates(crates);
}

fn get_crates_from_line(line: &str) -> CrateCells {
    parsing::map_line_to_chunk_vec(line, 4)
        .into_iter()
        .map(|create| {
            let ident = create[1];
            if ident.is_whitespace() {
                None
            } else {
                Some(ident)
            }
        })
        .collect()
}
