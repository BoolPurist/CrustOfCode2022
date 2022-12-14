use core::fmt::Debug;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub enum ExtraxtSeqRegexError {
    RegexPatternError(regex::Error),
    NoMatch,
    NoGroupFor(usize),
}
pub fn get_seq_from_regex<'a>(
    pattern: &str,
    input: &'a str,
    number_of_groups: usize,
) -> Result<Vec<&'a str>, ExtraxtSeqRegexError> {
    let mut output: Vec<&'a str> = Vec::with_capacity(number_of_groups);

    let re = Regex::new(pattern).map_err(|error| ExtraxtSeqRegexError::RegexPatternError(error))?;
    let captures = re
        .captures(input.trim())
        .ok_or(ExtraxtSeqRegexError::NoMatch)?;

    for index in 1..=number_of_groups {
        let to_add = captures
            .get(index)
            .ok_or(ExtraxtSeqRegexError::NoGroupFor(index))?;
        output.push(to_add.as_str())
    }

    Ok(output)
}

pub fn get_parsed_sep_by<T>(line: &str, sep: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as FromStr>::Err: Debug,
{
    line.split(sep)
        .map(|to_parse| to_parse.parse().expect("Could not parse to desired type"))
        .collect()
}

pub fn strip_away_left_part<'a>(strip_away_from: &'a str, prefix: &str) -> &'a str {
    strip_away_from
        .trim()
        .strip_prefix(prefix)
        .expect("Could not strip away")
}

pub type Lines<'a> = Vec<&'a str>;
pub fn split_lines_where_after<'a, P>(input: &'a str, perdicate: P) -> (Lines<'a>, Lines<'a>)
where
    P: Fn(&str) -> bool,
{
    split_lines_where(input, perdicate, true)
}

pub fn split_lines_where_before<'a, P>(input: &'a str, perdicate: P) -> (Lines<'a>, Lines<'a>)
where
    P: Fn(&str) -> bool,
{
    split_lines_where(input, perdicate, false)
}

pub fn split_chunks_where<'a, P>(input: &'a str, perdicate: P) -> Vec<Vec<&'a str>>
where
    P: Fn(&str) -> bool,
{
    let mut chunks = Vec::new();
    chunks.push(Vec::new());
    let mut counter = 0;

    for line in input.lines() {
        if perdicate(line) {
            chunks.push(Vec::new());
            counter += 1;
        } else {
            chunks[counter].push(line);
        }
    }

    chunks
}

fn split_lines_where<'a, P>(input: &'a str, perdicate: P, after: bool) -> (Lines<'a>, Lines<'a>)
where
    P: Fn(&str) -> bool,
{
    let lines: Lines<'_> = input.lines().collect();
    let where_split = match lines.iter().position(|line| perdicate(line)) {
        Some(position) => position + if after { 1 } else { 0 },
        None => return (lines.into_iter().collect(), Vec::new()),
    };

    let left = lines.into_iter().take(where_split);
    let right = input.lines().skip(where_split);

    (left.collect(), right.collect())
}

pub fn map_line_to_chunk_vec(line: &str, chunk_size: usize) -> Vec<Vec<char>> {
    line.chars()
        .collect::<Vec<char>>()
        .chunks(chunk_size)
        .map(|slice| slice.to_vec())
        .collect()
}
