use core::{cmp, num::ParseIntError, str::FromStr};

type PuzzelInput = Vec<(Assignment, Assignment)>;

pub fn calc_number_contained_assignment(input: &str) -> u32 {
    let parsed = parse_input(input);

    count_total_containments(&parsed)
}

pub fn calc_for_any_common_section(input: &str) -> u32 {
    let parsed = parse_input(input);

    count_any_section_containment(&parsed)
}

fn count_total_containments(parsed_input: &PuzzelInput) -> u32 {
    return parsed_input
        .into_iter()
        .fold(u32::default(), |total, pair| {
            let (left, right) = pair;
            let left_is_contained = get_one_or_zero(left, right);
            let right_is_contained = get_one_or_zero(right, left);

            // One line should yield still 1 even in case that two section groups overlap  each
            // other
            total + cmp::max(left_is_contained, right_is_contained)
        });

    fn get_one_or_zero(one: &Assignment, other: &Assignment) -> u32 {
        if one.is_contained_fully_by(other) {
            1u32
        } else {
            0u32
        }
    }
}

fn count_any_section_containment(parsed_input: &PuzzelInput) -> u32 {
    parsed_input
        .into_iter()
        .fold(u32::default(), |total, pair| {
            let (left, right) = pair;
            let to_add = if left.has_any_common_section(right) {
                1u32
            } else {
                0u32
            };

            total + to_add
        })
}

fn parse_input(input: &str) -> PuzzelInput {
    return input
        .lines()
        .map(|line| {
            let mut left_right = line.split(",");
            match (left_right.next(), left_right.next()) {
                (Some(first), Some(second)) => {
                    match (first.parse::<Assignment>(), second.parse::<Assignment>()) {
                        (Ok(fst_parsed), Ok(snd_parsed)) => (fst_parsed, snd_parsed),
                        _ => {
                            panic!("For left or right elf start and end section could be extracted")
                        }
                    }
                }
                _ => panic!("no left or right elf found during parsing"),
            }
        })
        .collect();
}

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn new(start: u32, end: u32) -> Self {
        Assignment { start, end }
    }

    fn is_contained_fully_by(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn has_any_common_section(&self, other: &Self) -> bool {
        let self_common_from_start = is_in_interval(self.start, other);
        let self_common_from_end = is_in_interval(self.end, other);
        let other_common_from_start = is_in_interval(other.start, self);
        let other_common_from_end = is_in_interval(other.end, self);

        return self_common_from_start
            || self_common_from_end
            || other_common_from_start
            || other_common_from_end;

        fn is_in_interval(point: u32, interval: &Assignment) -> bool {
            point >= interval.start && point <= interval.end
        }
    }
}

impl FromStr for Assignment {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_end = s.split("-");
        let left = start_end.next().expect("no start found");
        let right = start_end.next().expect("no end found");
        let start: u32 = left.parse()?;
        let end: u32 = right.parse()?;

        Ok(Assignment::new(start, end))
    }
}
