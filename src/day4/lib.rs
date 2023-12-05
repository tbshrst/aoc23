#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use itertools::Itertools;
#[cfg(test)]
use anyhow::Result;

#[cfg(test)]
#[derive(Debug)]
struct ScratchCard {
    card_number: u8,
    power: u32,
    num_wins: u8,
    count: u32,
}

#[cfg(test)]
fn eval_scratchcard(scratchcard: String) -> ScratchCard
{
    let (card, numbers) = scratchcard.split_once(':').unwrap();
    let card_number = String::from(card)
        .split_whitespace()
        .nth(1).unwrap()
        .parse::<u8>().unwrap();

    let n: Vec<_> = numbers
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();
    let mut s = n.split(|str| str == "|");

    let lhs = s.next().unwrap();
    let rhs = s.next().unwrap();
    let cross = lhs.iter().cartesian_product(rhs.iter());

    let num_wins = cross
        .into_iter()
        .filter(|(a, b)| a == b)
        .count() as u8;

    let power = if num_wins == 0 {
        0
    } else {
        1 << (num_wins - 1)
    };

    let count = if num_wins > 0 {
        1
    } else {
        0
    };

    ScratchCard {
        card_number,
        power,
        num_wins,
        count,
    }
}

#[cfg(test)]
fn eval_cascading_scratchcards(scratchcards: Vec<String>) -> u64
{
    let mut m: HashMap<u8, ScratchCard> = HashMap::new();

    for sc in scratchcards.into_iter() {
        let sc = eval_scratchcard(sc.to_string());
        println!("{:#?}", sc);
        m.insert(sc.card_number, sc);
    }

    let mut total_cards = 0;
    for i in 1..=m.len() {
        println!("i: {i}");
        let sc = m.get_mut(&(i as u8)).unwrap();
        let sc_wins = sc.num_wins;
        let sc_id = sc.card_number;
        let sc_count = sc.count as u64;

        println!("instances: {}", sc.count);
        println!("num wins: {}", sc_wins);
        total_cards += sc.count as u64;

        for i in 1..=sc_wins {
            let r = 1..=sc_count;
            println!("copies: {:#?}", r.clone().count()-1);
            for _ in r {
                let idx = sc_id + i;
                println!("copy card {idx}");
                let v = m.get_mut(&idx).unwrap();
                v.count += 1;
            }
        }
        println!("{:#?}", &m);
        println!("total cards: {total_cards}");
        println!("XXX");
    }

    total_cards
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib::{self, timer};

    #[test]
    fn test_scratchcards_example_1()
    {
        let _t = timer::Timer::new();
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
            "#;
        let input = lib::to_line_vector(input.to_string());
        let input = lib::filter_empty_lines(input);

        let mut power = 0;
        for line in input {
            let card = eval_scratchcard(line);
            power += card.power;
        }
        assert_eq!(power, 13);
    }

    #[test]
    fn test_scratchcards_task_1() -> Result<()>
    {
        let _t = timer::Timer::new();
        let input = lib::read_vectored_input("input.txt")?;
        let input = lib::filter_empty_lines(input);

        let mut power = 0;
        for line in input {
            let card = eval_scratchcard(line);
            power += card.power;
        }
        assert_eq!(power, !4294952090_u32);
        Ok(())
    }

    #[test]
    fn test_scratchcards_example_2()
    {
        let _t = timer::Timer::new();
        let input = r#"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
            "#;
        let input = lib::to_line_vector(input.to_string());
        let input = lib::filter_empty_lines(input);

        let num_cards = eval_cascading_scratchcards(input);
        assert_eq!(num_cards, 30);
    }

    // #[test]
    // fn test_scratchcards_task_2() -> Result<()>
    // {
    //     let _t = timer::Timer::new();
    //     let input = lib::read_vectored_input("input.txt")?;
    //     let input = lib::filter_empty_lines(input);

    //     let num_cards = eval_cascading_scratchcards(input);
    //     assert_eq!(num_cards, !4294952090_u64);
    //     Ok(())
    // }
}
