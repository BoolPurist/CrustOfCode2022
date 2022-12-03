use std::collections::HashSet;
use std::ops;

#[derive(Debug)]
struct Compartment(String);
#[derive(Debug)]
struct Rucksack(Compartment, Compartment);
type AllRucksacks = Vec<Rucksack>;

const UPPER_CASE_NUM_OFFSET: u32 = 27u32;
const LOWER_CASE_NUM_OFFSET: u32 = 1u32;
const UPPER_A_NUM: u32 = 'A' as u32;
const LOWER_A_NUM: u32 = 'a' as u32;

pub fn get_total_prio_of_dups(input: &str) -> u32 {
    let parsed_input = parse_input(input);
    let total = calc_duplicate_prios(&parsed_input);
    total.0
}

fn calc_duplicate_prios(calc_from: &AllRucksacks) -> Prio {
    calc_from
        .into_iter()
        .fold(Prio::default(), |total, next_rucksack| {
            let mut add_to_total = Prio::default();
            let duplicates = find_first_duplicate(next_rucksack);
            for one_dup in duplicates.0 {
                let numeric_dup = convert_letter_to_char(one_dup);
                add_to_total += numeric_dup.expect("Could convert char");
            }

            println!("{:?}", add_to_total);

            total + add_to_total
        })
}

type FoundCompartments = HashSet<char>;
#[derive(Debug)]
struct FoundDuplicates(Vec<char>);
fn find_first_duplicate(to_find_in: &Rucksack) -> FoundDuplicates {
    let left_compartment = &to_find_in.0;
    let right_compartment = &to_find_in.1;

    let found_in_left = find_uniques_comparment(left_compartment);
    let found_in_right = find_uniques_comparment(right_compartment);

    let duplicates: FoundDuplicates = FoundDuplicates(
        found_in_left
            .intersection(&found_in_right)
            .cloned()
            .collect(),
    );

    return duplicates;

    fn find_uniques_comparment(compartment: &Compartment) -> FoundCompartments {
        let mut found: FoundCompartments = HashSet::new();
        for current_item in compartment.0.chars() {
            _ = !found.insert(current_item);
        }

        found
    }
}

#[derive(Debug)]
struct Prio(u32);
#[derive(Debug)]
enum PrioConvertError {
    IsNotAscii,
    NotUpperOrLowerCaseLetter,
}
fn convert_letter_to_char(to_convert: char) -> Result<Prio, PrioConvertError> {
    if !to_convert.is_ascii() {
        return Err(PrioConvertError::IsNotAscii);
    }
    let numeric_value = to_convert as u32;
    if to_convert.is_lowercase() {
        Ok(Prio(
            numeric_value - (LOWER_A_NUM as u32) + LOWER_CASE_NUM_OFFSET,
        ))
    } else if to_convert.is_uppercase() {
        Ok(Prio(
            numeric_value - (UPPER_A_NUM as u32) + UPPER_CASE_NUM_OFFSET,
        ))
    } else {
        Err(PrioConvertError::NotUpperOrLowerCaseLetter)
    }
}

impl ops::Add<Prio> for Prio {
    type Output = Prio;
    fn add(self, rsh: Prio) -> Self::Output {
        Self(self.0 + rsh.0)
    }
}

impl ops::AddAssign<Prio> for Prio {
    fn add_assign(&mut self, rhs: Prio) {
        self.0 = self.0 + rhs.0;
    }
}

impl Default for Prio {
    fn default() -> Self {
        Self(0)
    }
}

fn parse_input(to_convert: &str) -> AllRucksacks {
    let mut parsed: AllRucksacks = Vec::new();

    for next_line in to_convert.lines() {
        let half_number_letters = next_line.len() / 2usize;
        let left_compartment: String = next_line.chars().take(half_number_letters).collect();
        let right_compartment: String = next_line.chars().skip(half_number_letters).collect();
        parsed.push(Rucksack(
            Compartment(left_compartment),
            Compartment(right_compartment),
        ));
    }

    parsed
}
