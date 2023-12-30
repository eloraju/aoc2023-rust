use crate::{Solution, SolutionPair};
use std::{env, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let path_to_input = "input/02.txt";
    let full_path = env::current_dir().unwrap().join(path_to_input);
    let input = read_to_string(full_path).unwrap();
    // Your solution here...
    let sol1: u32 = part1(&input);
    let sol2: u32 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &str) -> u32 {
    let r_max = 12;
    let g_max = 13;
    let b_max = 14;

    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|g| Game::new(g))
        .filter(|game| game.r_max <= r_max && game.g_max <= g_max && game.b_max <= b_max)
        .fold(0, |acc, game| acc + game.id)
}

fn part2(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|g| Game::new(g))
        .fold(0, |acc, game| game.power + acc)
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
    r_max: u32,
    g_max: u32,
    b_max: u32,
    input: String,
    power: u32,
}

#[derive(Debug)]
struct Round {
    r: u32,
    g: u32,
    b: u32,
}

impl Game {
    pub fn new(game_str: &str) -> Self {
        let mut id_rounds = game_str.split(": ").collect::<Vec<&str>>();

        let rounds = id_rounds
            .pop()
            .unwrap()
            .split("; ")
            .map(|round| Round::new(round))
            .collect::<Vec<Round>>();

        let id = id_rounds
            .pop()
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut r_max = 0;
        let mut g_max = 0;
        let mut b_max = 0;

        rounds.iter().for_each(|round| {
            if round.r > r_max {
                r_max = round.r
            };
            if round.g > g_max {
                g_max = round.g
            };
            if round.b > b_max {
                b_max = round.b
            };
        });

        let power = r_max * g_max * b_max;

        Self {
            id,
            rounds,
            r_max,
            g_max,
            b_max,
            input: game_str.to_string(),
            power,
        }
    }
}

impl Round {
    pub fn new(round_str: &str) -> Self {
        let mut round = Round {
            r: 0,
            g: 0,
            b: 0,
        };

        round_str.split(", ").for_each(|val_key| {
            let num_key = val_key.split(" ").collect::<Vec<&str>>();
            match *num_key.last().unwrap() {
                "red" => {
                    round.r = num_key
                        .first()
                        .map(|num| num.parse::<u32>().unwrap_or(0))
                        .unwrap();
                }
                "green" => {
                    round.g = num_key
                        .first()
                        .map(|num| num.parse::<u32>().unwrap_or(0))
                        .unwrap();
                }
                "blue" => {
                    round.b = num_key
                        .first()
                        .map(|num| num.parse::<u32>().unwrap_or(0))
                        .unwrap();
                }
                &_ => (),
            }
        });
        round
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    const TEST_INPUT_2: &str = r"\
";

    #[test]
    fn p1_test() {
        let result = part1(TEST_INPUT_1);

        assert_eq!(result, 8);
    }

    #[test]
    fn p2_test() {}
}
