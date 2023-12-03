use core::fmt;
use std::{fmt::Display, ops::Index};

#[cfg(test)]
use anyhow::Result;

struct EngineSchematic {
    #[cfg(test)]
    rows: usize,
    #[cfg(test)]
    cols: usize,
    matrix: lib::Matrix2D,
}

impl EngineSchematic {
    #[cfg(test)]
    pub fn new(schema: Vec<String>) -> Self
    {
        let rows = schema.len();
        let cols = schema.first().unwrap().len();

        Self{
            rows,
            cols,
            matrix: lib::Matrix2D::new(schema)
        }
    }

    #[cfg(test)]
    pub fn get(&self, mut row: i16, mut col: i16) -> Option<&char>
    {
        if row < 0 {
            row = self.rows as i16 - row;
        }

        if col < 0 {
            col = self.cols as i16 - col;
        }

        if row as usize >= self.rows || col as usize >= self.cols {
            return None;
        }

        Some(self.matrix.index((row as usize, col as usize)))
    }

    #[cfg(test)]
    pub fn is_number(&self, row: i16, col: i16) -> bool
    {
        let value = self.get(row, col); 
        if value.is_none() || !value.unwrap().is_alphanumeric() {
            return false;
        }

        true
    }

    #[cfg(test)]
    pub fn get_number(&self, row: i16, col: i16) -> Option<u32>
    {
        self.get(row, col)?;

        let mut idx_start = col;
        let mut idx_end = col as usize;

        while idx_start > 0 && self.is_number(row, idx_start - 1) {            
            idx_start -= 1;
        }

        while idx_end < self.cols && self.is_number(row, (idx_end + 1) as i16) {            
                idx_end += 1;
        }

        let m_row = &self.matrix.data[row as usize];
        let maybe_number = &m_row[idx_start as usize ..=idx_end];

        let maybe_number = maybe_number
            .iter()
            .collect::<String>();
    
        maybe_number.parse::<u32>().ok()
    }

    #[cfg(test)]
    pub fn delete_number(&mut self, row: i16, col: i16)
    {
        let mut idx_start = col;
        let mut idx_end = col as usize;

        while idx_start > 0 && self.is_number(row, idx_start - 1) {            
            idx_start -= 1;
        }

        while idx_end < self.cols && self.is_number(row, (idx_end + 1) as i16) {            
                idx_end += 1;
        }

        let m_row = &mut self.matrix.data[row as usize];
        for idx in idx_start as usize ..=idx_end {
            m_row[idx] = '.';
        }   
    }

    #[cfg(test)]
    fn get_and_delete_number(&mut self, row: i16, col: i16) -> u32
    {
        match self.get_number(row, col) {
            Some(number) => {
                self.delete_number(row, col);
                number
            }, 
            None => 0
        }
    }

    #[cfg(test)]
    pub fn gather_numbers(&mut self) -> u32
    {
        let mut sum = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                let ch = self.get(row as i16, col as i16).unwrap();
                if *ch != '.' && !ch.is_alphanumeric() {
                    sum += self.get_and_delete_number((row-1) as i16, (col-1) as i16);
                    sum += self.get_and_delete_number((row-1) as i16, (col) as i16);
                    sum += self.get_and_delete_number((row-1) as i16, (col+1) as i16);
                    sum += self.get_and_delete_number((row) as i16, (col-1) as i16);
                    sum += self.get_and_delete_number((row) as i16, (col+1) as i16);
                    sum += self.get_and_delete_number((row+1) as i16, (col-1) as i16);
                    sum += self.get_and_delete_number((row+1) as i16, (col) as i16);
                    sum += self.get_and_delete_number((row+1) as i16, (col+1) as i16);
                }
            }
        }

        sum
    }

    #[cfg(test)]
    pub fn gather_gear_ratios(&mut self) -> u32
    {
        let mut total_sum = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                let ch = self.get(row as i16, col as i16).unwrap();
                if *ch == '*' {
                    let mut gear_ratio = 1;
                    let mut num_gears = 0;
                    let fields = [
                        ((row-1) as i16, (col-1) as i16),
                        ((row-1) as i16, (col) as i16),
                        ((row-1) as i16, (col+1) as i16),
                        ((row) as i16, (col-1) as i16),
                        ((row) as i16, (col+1) as i16),
                        ((row+1) as i16, (col-1) as i16),
                        ((row+1) as i16, (col) as i16),
                        ((row+1) as i16, (col+1) as i16),
                    ];

                    for (row, col) in fields {
                        let gear = self.get_and_delete_number(row, col);
                        if gear != 0 {
                            if num_gears >= 2 {
                                continue;
                            }
                            gear_ratio *= gear;
                            num_gears += 1;
                        }
                    }
                    if num_gears == 2 {
                        total_sum += gear_ratio;
                    }
                }
            }
        }

        total_sum
    }

}

impl Index<(usize, usize)> for EngineSchematic {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output
    {
        self.matrix.index(index)
    }
}

impl Display for EngineSchematic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        self.matrix.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib::{self, timer};

    #[test]
    fn test_engine_part_example_1() -> Result<()>
    {
        let _t = timer::Timer::new();
        let input = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            "#;
        let input = lib::to_line_vector(input.to_string());
        let input = lib::filter_empty_lines(input);

        let mut m = EngineSchematic::new(input);
        let sum = m.gather_numbers();

        assert_eq!(sum, 4361);
        Ok(())
    }

    #[test]
    fn test_engine_part_task_1() -> Result<()>
    {
        let _t = timer::Timer::new();
        let input = lib::read_vectored_input("input.txt")?;
        let input = lib::filter_empty_lines(input);

        let mut m = EngineSchematic::new(input);
        let sum = m.gather_numbers();

        assert_eq!(sum, !4294420732_u32);
        Ok(())
    }

    #[test]
    fn test_engine_part_example_2() -> Result<()>
    {
        let _t = timer::Timer::new();
        let input = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            "#;
        let input = lib::to_line_vector(input.to_string());
        let input = lib::filter_empty_lines(input);

        let mut m = EngineSchematic::new(input);
        let sum = m.gather_gear_ratios();

        assert_eq!(sum, 467835);
        Ok(())
    }

    #[test]
    fn test_engine_part_task_2() -> Result<()>
    {
        let _t = timer::Timer::new();
        let input = lib::read_vectored_input("input.txt")?;
        let input = lib::filter_empty_lines(input);

        let mut m = EngineSchematic::new(input);
        let sum = m.gather_gear_ratios();

        assert_eq!(sum, !4203935921_u32);
        Ok(())
    }
}
