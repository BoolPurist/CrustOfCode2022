use crate::challenge_args::ChallangeArgs;
use std::num::ParseIntError;

type OneElveCal = Vec<u32>;
type Calories = Vec<OneElveCal>;

pub fn solve_challenge(args: &ChallangeArgs) -> Result<u32, ParseIntError> {
    let grouped_cals = parse_input(&args.input)?;

    Ok(get_max_calorie(&grouped_cals))
}

pub fn get_total_cal_top(args: &ChallangeArgs, number_top: usize) -> Result<u32, ParseIntError> {
    let grouped_cals = parse_input(&args.input)?;

    let mut calor_sum: OneElveCal = grouped_cals
        .into_iter()
        .map(|elve_calor| elve_calor.into_iter().sum())
        .collect();

    calor_sum.sort();
    calor_sum.reverse();

    Ok(calor_sum.into_iter().take(number_top).sum())
}

fn get_max_calorie(get_max_from: &Calories) -> u32 {
    return get_max_from.into_iter().fold(0u32, |akk, elve_calor| {
        let sum_calorien = elve_calor.into_iter().sum();

        if akk < sum_calorien {
            sum_calorien
        } else {
            akk
        }
    });
}

fn parse_input(input: &str) -> Result<Calories, ParseIntError> {
    let mut parsed: Calories = Vec::new();
    let mut current_elve: OneElveCal = Vec::new();

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed == "" {
            parsed.push(current_elve);
            current_elve = Vec::new();
        } else {
            let item: u32 = match trimmed.parse() {
                Ok(value) => value,
                Err(error) => return Err(error),
            };

            current_elve.push(item);
        }
    }

    if !current_elve.is_empty() {
        parsed.push(current_elve);
    }

    Ok(parsed)
}
