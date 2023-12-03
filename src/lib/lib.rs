pub mod timer;

use std::{fs,
    fmt::{Display, self},
    io::Error,
    ops::Index,
};

pub fn open_file(filepath: &str) -> Result<String, Error>
{
    fs::read_to_string(filepath)
}

pub fn read_raw_input(filepath: &str) -> Result<String, Error>
{
    open_file(filepath)
}

pub fn read_vectored_input(filepath: &str) -> Result<Vec<String>, Error>
{
    Ok(to_line_vector(open_file(filepath)?))
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


pub struct Matrix2D {
    pub data: Vec<Vec<char>>,
}

impl Matrix2D {
    pub fn new(data: Vec<String>) -> Self
    {
        let data = data
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        Self {
            data,
        }
    }
}

impl Index<(usize, usize)> for Matrix2D {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output
    {
        &self.data[index.0][index.1]
    }
}

impl Display for Matrix2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for row in &self.data {
            writeln!(f, "|{}|", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}
