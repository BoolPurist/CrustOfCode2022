use clap::Parser;
use solution_advent_of_code_2022::{challenge_args::ChallangeArgs, day_01, day_02, day_03};

use std::fs;
use std::io;

fn main() {
    let mut args: ChallangeArgs = ChallangeArgs::parse();
    read_file_if_needed(&mut args).expect("Could read not input from file.");
    solve_for_certain_day(&args);
}

fn solve_for_certain_day(args: &ChallangeArgs) {
    match args.day {
        1 => match args.task {
            1 => println!(
                "Most calories: {}",
                day_01::get_top_calorie(&args).expect("Error in parsing input")
            ),
            2 => println!(
                "Calories of top 3 evles: {}",
                day_01::get_total_cal_top(args, 3).expect("Error in parsing input")
            ),
            invalid_task => abort_for_invalid_task(invalid_task),
        },
        2 => match args.task {
            1 => {
                let total = day_02::calc_score_of_strat(&args.input);
                println!("The score following the strategy is: {}", total);
            }
            2 => {
                let total = day_02::calc_score_outcome_strat(&args.input);
                println!("The score following the outcome strategy is: {}", total);
            }
            invalid_task => abort_for_invalid_task(invalid_task),
        },
        3 => match args.task {
            1 => {
                let total = day_03::get_total_prio_of_dups(&args.input);
                println!("The total of priotities of the duplicates in the rucksacks: {total}");
            }
            2 => {
                let total = day_03::get_total_prio_of_group_badges(&args.input);
                println!("The total of prios of group badges: {total}");
            }
            invalid_task => abort_for_invalid_task(invalid_task),
        },
        unknown_day => abort_for_invalid_day(unknown_day),
    }
}

fn read_file_if_needed(args: &mut ChallangeArgs) -> Result<(), io::Error> {
    if !args.input_as_path {
        return Ok(());
    }

    let file_content = fs::read_to_string(&args.input)?;
    args.input = file_content;

    Ok(())
}

fn abort_for_invalid_day(unknown_day: u32) {
    eprint!("No solution for day with number: {unknown_day}");
    std::process::exit(1);
}

fn abort_for_invalid_task(unkown_task: u32) {
    eprint!("No solution for task with number: {unkown_task}");
    std::process::exit(2);
}
