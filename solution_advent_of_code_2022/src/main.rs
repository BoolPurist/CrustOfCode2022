use clap::Parser;
use solution_advent_of_code_2022::challenge_args::ChallangeArgs;
use std::fs;
use std::io;

fn main() {
    let mut args: ChallangeArgs = ChallangeArgs::parse();
    read_file_if_needed(&mut args).expect("Could read not input from file.");

    println!("{:?}", args);
}

fn read_file_if_needed(args: &mut ChallangeArgs) -> Result<(), io::Error> {
    if !args.input_as_path {
        return Ok(());
    }

    let file_content = fs::read_to_string(&args.input)?;
    args.input = file_content;

    Ok(())
}
