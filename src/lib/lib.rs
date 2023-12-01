pub mod timer;

use std::{fs, io::Error};

pub fn read_input(filepath: &str) -> Result<Vec<String>, Error>
{
    Ok(to_line_vector(fs::read_to_string(filepath)?))
}

pub fn to_line_vector(multiline: String) -> Vec<String>
{
    multiline
        .split('\n')
        .map(String::from)
        .map(|string| string.trim().to_owned())
        .collect::<Vec<String>>()
}

pub fn filter_empty_lines(vec: Vec<String>) -> Vec<String>
{
    vec
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect()
}
