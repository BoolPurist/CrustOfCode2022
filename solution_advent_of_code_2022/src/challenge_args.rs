use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ChallangeArgs {
    /// The input to which a result is to be calculated
    pub input: String,
    /// If provided, the parameter input will be treated as a path to file.
    /// the content of the file is used as input.
    #[arg(short, long)]
    pub input_as_path: bool,
    /// Number of day under which the given task is given
    #[arg(short, long)]
    pub day: u32,
    /// Which task is given under a given day
    #[arg(short, long)]
    pub task: u32,
}
