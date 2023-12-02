#[cfg(test)]
use anyhow::{Result, bail};
#[cfg(test)]
use std::{cmp::Ordering, vec};

#[derive(Debug)]
struct KubeConfig {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl PartialEq for KubeConfig {
    fn eq(&self, other: &KubeConfig) -> bool
    {
        self.red == other.red &&
        self.green == other.green &&
        self.blue == other.blue
    }
}

#[cfg(test)]
impl PartialOrd for KubeConfig {
    fn partial_cmp(&self, other: &KubeConfig) -> std::option::Option<std::cmp::Ordering>
    {
        if self == other {
            Some(Ordering::Equal)
        } else if self.red <= other.red &&
            self.green <= other.green &&
            self.blue <= other.blue
        {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

#[derive(Debug)]
struct Game {
    #[cfg(test)]
    pub id: u8,
    #[cfg(test)]
    pub rounds: Vec<KubeConfig>,
}

impl Game {
    #[cfg(test)]
    fn new(game_description: String) -> Result<Self>
    {
        let game_description = game_description.split_once(':');
        assert!(game_description.is_some());

        let (id, rounds) = match game_description {
            Some((id, rounds)) => {
                let id = match id.split_once(' ') {
                    Some((_, id)) => id,
                    None => bail!("error splitting")
                };
                (id, rounds)
            },
            None => bail!("error splitting"),
        };
        let id = id.parse::<u8>()
            .expect("shouldve been parsable");

        let rounds = {
            let mut vec: Vec<KubeConfig> = vec![];

            for round in rounds.split(';') {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                for kube in round.split(',') {
                    let (amount, color) = match kube.trim().split_once(' ') {
                        Some((amount, color)) => {
                            assert!(!amount.is_empty());
                            assert!(!color.is_empty());
                            (amount, color)
                        },
                        None => bail!("error splitting"),
                    };

                    let amount = amount.parse::<u8>()
                        .expect("shouldve been parsable");

                    match color {
                        "red" => red = amount,
                        "green" => green = amount,
                        "blue" => blue = amount,
                        _ => bail!("unknown color"),
                    }
                }
                vec.push(KubeConfig {red, green, blue});
            }

            vec
        };

        Ok(Self {
            id,
            rounds,
        })
    }

    #[cfg(test)]
    fn is_possible(&self, config: &KubeConfig) -> bool
    {
        for round in &self.rounds {
            if round > config {
                return false;
            }
        }

        true
    }

    #[cfg(test)]
    fn parse_games(input: Vec<String>) -> Result<Vec<Game>>
    {
        let mut games: Vec<Game> = vec![];
        for game_description in input {
            games.push(Game::new(game_description)?)
        }

        Ok(games)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib::timer;

    #[test]
    fn test_kube_game_example_1() -> Result<()>
    {
        let _t = timer::Timer::new();
        let input = r#"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            "#;
        let input = lib::to_line_vector(input.to_string());
        let input = lib::filter_empty_lines(input);

        let games = Game::parse_games(input)?;
        let challenge = KubeConfig{red: 12, green: 13, blue: 14};
        let sum_ids = games.into_iter()
            .filter(|g| g.is_possible(&challenge))
            .fold(0, |acc, g| acc + g.id);

        assert_eq!(sum_ids, 8);
        Ok(())
    }

    #[test]
    fn test_kube_game_task_1() -> Result<()>
    {
        let _t = timer::Timer::new();
        let input = lib::read_input("input.txt")?;
        let vec = lib::filter_empty_lines(input);

        let games = Game::parse_games(vec)?;
        let challenge = KubeConfig{red: 12, green: 13, blue: 14};
        let sum_ids = games.into_iter()
            .filter(|g| g.is_possible(&challenge))
            .fold(0_u32, |acc, g| acc + g.id as u32);

        assert_eq!(sum_ids, !4294965146_u32);
        Ok(())
    }

    #[test]
    fn test_kube_game_example_2() -> Result<()>
    {
        let _t = timer::Timer::new();
        let input = r#"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            "#;
        let input = lib::to_line_vector(input.to_string());
        let input = lib::filter_empty_lines(input);

        let mut total_summed_power: u64 = 0;
        let games = Game::parse_games(input)?;
        for game in games {
            let (max_red, max_green, max_blue) = game.rounds
                .iter()
                .fold((0, 0, 0),
                    |(r, g, b), config| {
                        (r.max(config.red), g.max(config.green), b.max(config.blue))
                    });

            total_summed_power += max_red as u64 * max_green as u64 * max_blue as u64;
        }

        assert_eq!(total_summed_power, 2286);
        Ok(())
    }

    #[test]
    fn test_kube_game_task_2() -> Result<()>
    {
        let _t = timer::Timer::new();
        let input = lib::read_input("input.txt")?;
        let input = lib::filter_empty_lines(input);

        let mut total_summed_power = 0;
        let games = Game::parse_games(input)?;
        for game in games {
            let (max_red, max_green, max_blue) = game.rounds
                .iter()
                .fold((0, 0, 0),
                    |(r, g, b), config| {
                        (r.max(config.red), g.max(config.green), b.max(config.blue))
                    });

            total_summed_power += max_red as u32 * max_green as u32 * max_blue as u32;
        }

        assert_eq!(total_summed_power, !4294896021_u32);
        Ok(())
    }
}
