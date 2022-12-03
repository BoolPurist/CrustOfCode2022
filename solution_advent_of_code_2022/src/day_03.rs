use std::collections::HashSet;
use std::ops;

type AllRucksacks = Vec<Rucksack>;
type FoundItems = HashSet<char>;
type RucksackNoComparment = String;
type Compartment = String;

#[derive(Debug)]
struct FoundGroupBadges(Vec<FoundItemDuplicates>);
#[derive(Debug)]
struct Rucksack(Compartment, Compartment);
#[derive(Debug)]
struct RucksackGroup(Vec<[RucksackNoComparment; 3]>);
#[derive(Debug)]
struct FoundItemDuplicates(Vec<char>);

const UPPER_CASE_NUM_OFFSET: u32 = 27u32;
const LOWER_CASE_NUM_OFFSET: u32 = 1u32;
const UPPER_A_NUM: u32 = 'A' as u32;
const LOWER_A_NUM: u32 = 'a' as u32;

pub fn get_total_prio_of_dups(input: &str) -> u32 {
    let parsed_input = parse_input(input);
    let total = calc_duplicate_prios(&parsed_input);
    total.0
}

pub fn get_total_prio_of_group_badges(input: &str) -> u32 {
    let input_parsed = parse_input(input);
    let grouped = transform_input_for_group_badges(input_parsed);
    let group_badges = get_found_group_item(grouped);
    let total = get_total_prio_grouped(&group_badges);

    total.0
}

fn get_total_prio_grouped(group_badges: &FoundGroupBadges) -> Prio {
    group_badges
        .0
        .iter()
        .fold(Prio::default(), |total, badges| {
            let mut local_total = Prio::default();
            for one_badge in &badges.0 {
                let prio =
                    convert_letter_to_prio(*one_badge).expect("Could not convert char to prio");
                local_total += prio;
            }
            total + local_total
        })
}

fn get_found_group_item(find_in: RucksackGroup) -> FoundGroupBadges {
    FoundGroupBadges(
        find_in
            .0
            .into_iter()
            .map(|grouped| {
                let found_in_1 = find_uniques_letters(&grouped[0]);
                let found_in_2 = find_uniques_letters(&grouped[1]);
                let found_in_3 = find_uniques_letters(&grouped[2]);

                FoundItemDuplicates(
                    found_in_1
                        .intersection(&found_in_2)
                        .cloned()
                        .collect::<FoundItems>()
                        .intersection(&found_in_3)
                        .cloned()
                        .collect(),
                )
            })
            .collect(),
    )
}

fn transform_input_for_group_badges(to_group: AllRucksacks) -> RucksackGroup {
    RucksackGroup(
        to_group
            .chunks(3)
            .map(|group_rucksack| {
                if let [first, second, third] = group_rucksack {
                    let combined_first = combine_compartments(&first.0, &first.1);
                    let combined_second = combine_compartments(&second.0, &second.1);
                    let combined_third = combine_compartments(&third.0, &third.1);

                    return [combined_first, combined_second, combined_third];
                }

                panic!("Every item as a slice should have 3 elements")
            })
            .collect(),
    )
}

fn combine_compartments(left: &Compartment, right: &Compartment) -> Compartment {
    let mut left_owned = String::from(left);
    left_owned.push_str(&right);
    left_owned
}

fn calc_duplicate_prios(calc_from: &AllRucksacks) -> Prio {
    calc_from
        .into_iter()
        .fold(Prio::default(), |total, next_rucksack| {
            let mut add_to_total = Prio::default();
            let duplicates = find_first_duplicate(next_rucksack);
            for one_dup in duplicates.0 {
                let numeric_dup = convert_letter_to_prio(one_dup);
                add_to_total += numeric_dup.expect("Could convert char");
            }

            total + add_to_total
        })
}

fn find_first_duplicate(to_find_in: &Rucksack) -> FoundItemDuplicates {
    let left_compartment = &to_find_in.0;
    let right_compartment = &to_find_in.1;

    let found_in_left = find_uniques_letters(&left_compartment);
    let found_in_right = find_uniques_letters(&right_compartment);

    let duplicates: FoundItemDuplicates = FoundItemDuplicates(
        found_in_left
            .intersection(&found_in_right)
            .cloned()
            .collect(),
    );

    return duplicates;
}

fn find_uniques_letters(compartment: &str) -> FoundItems {
    let mut found: FoundItems = HashSet::with_capacity(compartment.len());
    found.extend(compartment.chars());
    found
}

#[derive(Debug)]
struct Prio(u32);
#[derive(Debug)]
enum PrioConvertError {
    IsNotAscii,
    NotUpperOrLowerCaseLetter,
}
fn convert_letter_to_prio(to_convert: char) -> Result<Prio, PrioConvertError> {
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
    to_convert
        .lines()
        .map(|next_line| {
            let half_number_letters = next_line.len() / 2usize;
            let left_compartment: String = next_line.chars().take(half_number_letters).collect();
            let right_compartment: String = next_line.chars().skip(half_number_letters).collect();

            Rucksack(left_compartment, right_compartment)
        })
        .collect()
}
