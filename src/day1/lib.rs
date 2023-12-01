pub fn filter_outer_digits(vec: Vec<String>) -> Vec<String>
{
    vec
        .into_iter()
        .map(|mut str| {
            str.retain(|c| c.is_ascii_digit());
            str
        })
        .map(|s|
            vec![
                s.chars().next().unwrap(),
                s.chars().next_back().unwrap(),
            ]
            .into_iter()
            .collect())
        .collect::<Vec<String>>()
}

pub fn sum_outer_digits(vec: &Vec<String>) -> u32
{
    vec
        .iter()
        .map(|s| s.parse::<u32>().unwrap_or_default())
        .sum::<u32>()
}

pub fn words_to_digits(vec: Vec<String>) -> Vec<String>
{
    vec
        .into_iter().map(|string|
            string.replace("zero", "z0o")
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .replace("ten", "t10n")
                .replace("eleven", "e11n")
                .replace("twelve", "t12n")
                .replace("thirteen", "t13n")
                .replace("fourteen", "f14n")
                .replace("fifteen", "f15n")
                .replace("sixteen", "s16n")
                .replace("seventeen", "s17n")
                .replace("eighteen", "e18n")
                .replace("nineteen", "n19n"))
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use lib::timer;

    #[test]
    fn test_sum_outer_digits_example_1()
    {
        let _t = timer::Timer::new();
        let input = r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
            "#;
        let input = lib::to_line_vector(input.to_string());
        let vec = lib::filter_empty_lines(input);
        let vec = filter_outer_digits(vec);
        let sum = sum_outer_digits(&vec);
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_sum_outer_digits_task_1() -> Result<()>
    {
        let _t = timer::Timer::new();
        let input = lib::read_input("input.txt")?;
        let vec = lib::filter_empty_lines(input);
        let vec = filter_outer_digits(vec);
        let sum = sum_outer_digits(&vec);
        assert_eq!(sum, 54968);
        Ok(())
    }

    #[test]
    fn test_sum_outer_digits_example_2()
    {
        let _t = timer::Timer::new();
        let input = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            "#;
        let input = lib::to_line_vector(input.to_string());
        let vec = lib::filter_empty_lines(input);
        let vec = words_to_digits(vec);
        let vec = filter_outer_digits(vec);
        let sum = sum_outer_digits(&vec);
        assert_eq!(sum, 281);
    }

    #[test]
    fn test_sum_outer_digits_task_2() -> Result<()>
    {
        let _t = timer::Timer::new();
        let input = lib::read_input("input.txt")?;
        let vec = lib::filter_empty_lines(input);
        let vec = words_to_digits(vec);
        let vec = filter_outer_digits(vec);
        let sum = sum_outer_digits(&vec);
        assert_eq!(sum, 54094);
        Ok(())
    }
}
