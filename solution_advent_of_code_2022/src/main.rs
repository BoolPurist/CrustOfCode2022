use clap::Parser;
use solution_advent_of_code_2022::{
    challenge_args::ChallangeArgs, day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08
};

use std::fs;
use std::io;

const TASK_ONE: u32 = 1;
const TASK_TWO: u32 = 2;
const TASK_THREE: u32 = 3;

const DAY_01: u32 = 1;
const DAY_02: u32 = 2;
const DAY_03: u32 = 3;
const DAY_04: u32 = 4;
const DAY_05: u32 = 5;
const DAY_06: u32 = 6;
const DAY_07: u32 = 7;
const DAY_08: u32 = 8;

fn main() {
    let mut args: ChallangeArgs = ChallangeArgs::parse();
    read_file_if_needed(&mut args).expect("Could read not input from file.");
    solve_for_certain_day(&args);
}

fn solve_for_certain_day(args: &ChallangeArgs) {
    match args.day {
        DAY_01 => match args.task {
            TASK_ONE => println!(
                "Most calories: {}",
                day_01::get_top_calorie(&args).expect("Error in parsing input")
            ),
            TASK_TWO => println!(
                "Calories of top 3 evles: {}",
                day_01::get_total_cal_top(args, 3).expect("Error in parsing input")
            ),
            invalid_task => abort_for_invalid_task(invalid_task),
        },
        DAY_02 => match args.task {
            TASK_ONE => {
                let total = day_02::calc_score_of_strat(&args.input);
                println!("The score following the strategy is: {}", total);
            }
            TASK_TWO => {
                let total = day_02::calc_score_outcome_strat(&args.input);
                println!("The score following the outcome strategy is: {}", total);
            }
            invalid_task => abort_for_invalid_task(invalid_task),
        },
        DAY_03 => match args.task {
            TASK_ONE => {
                let total = day_03::get_total_prio_of_dups(&args.input);
                println!("The total of priotities of the duplicates in the rucksacks: {total}");
            }
            TASK_TWO => {
                let total = day_03::get_total_prio_of_group_badges(&args.input);
                println!("The total of prios of group badges: {total}");
            }
            invalid_task => abort_for_invalid_task(invalid_task),
        },
        DAY_04 => match args.task {
            TASK_ONE => {
                let number_fully_contained = day_04::calc_number_contained_assignment(&args.input);
                println!(
                    "Number of section fully contained by another: {}",
                    number_fully_contained
                );
            }
            TASK_TWO => {
                let number_any_common_section = day_04::calc_for_any_common_section(&args.input);
                println!(
                    "Number lines with any commond section: {}",
                    number_any_common_section
                );
            }
            invalid_task => abort_for_invalid_task(invalid_task),
        },
        DAY_05 => match args.task {
            TASK_ONE => {
                let list_of_top_9000 = day_05::get_tops_stack_9000(&args.input);
                println!("All letters after 9000 {}", list_of_top_9000);
            }
            TASK_TWO => {
                let list_of_top_9001 = day_05::get_tops_stack_9001(&args.input);
                println!("All letters after 9001 {}", list_of_top_9001);
            }
            invalid_task => abort_for_invalid_task(invalid_task),
        },
        DAY_06 => match args.task {
            TASK_ONE => {
                let start_of_first_packe = day_06::get_end_of_first_packet_start(&args.input, 4);
                println!("Start marker of 1. packet ends at {}", start_of_first_packe);
            }
            TASK_TWO => {
                let start_of_first_message = day_06::get_end_of_first_packet_start(&args.input, 14);
                println!(
                    "Start marker of 1. message ends at {}",
                    start_of_first_message
                );
            }
            invalid_task => abort_for_invalid_task(invalid_task),
        },
        DAY_07 => match args.task {
            TASK_ONE => {
                const MAX: usize = 100000;
                let number_with_at_most = day_07::get_number_size_at_most(&args.input, MAX);

                println!(
                    "Number of directory not greater than {}: {}",
                    MAX, number_with_at_most
                );
            }
            TASK_TWO => {
                const TOTAL_FREE: day_07::SizeOfFile = 70_000_000;
                const NEEDED_FREE: day_07::SizeOfFile = 30_000_000;

                let (name, freed, needed) =
                    day_07::get_directory_to_delete(&args.input, TOTAL_FREE, NEEDED_FREE);

                println!("For the update {} bytes needed to freed.\nDirectory {} needs to be freed for {}", 
                         needed, 
                         name, 
                         freed
                         );
            },
            TASK_THREE => {
              let file_layout = day_07::draw_file_system(&args.input);

              println!("{file_layout}")
            },
            invalid_task => abort_for_invalid_task(invalid_task),
        },
        DAY_08 => match args.task {
            TASK_ONE => {
                let visible_trees_number = day_08::get_number_of_visible(&args.input);
                println!("Number of visible trees {}", visible_trees_number);
            }
            TASK_TWO => {
                let highest_score = day_08::get_max_scenic_score(&args.input);
                println!("Highes scenic score: {}", highest_score );
            },
            invalid_task => abort_for_invalid_task(invalid_task),
        } 
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
