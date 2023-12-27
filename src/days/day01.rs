use crate::{Solution, SolutionPair};
use regex::Regex;
use std::{collections::HashMap, env, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let path_to_input = "input/01.txt";
    let full_path = env::current_dir().unwrap().join(path_to_input);
    println!("path: {}", full_path.to_string_lossy());
    let input = read_to_string(full_path).unwrap();
    let sol1: u32 = p1(&input);
    let sol2: u32 = p2(&input).iter().sum();

    (Solution::from(sol1), Solution::from(sol2))
}

// 0.25ms
fn p1(data: &str) -> u32 {
    return data
        .split("\n")
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            let numbers = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            return format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
                .parse::<u32>()
                .ok();
        })
        .sum();
}

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    value: u32,
    is_leaf: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for char in word.chars() {
            node = node.children.entry(char).or_insert(TrieNode::default());
        }
        node.is_leaf = true;
        node.value = get_str_as_int(word)
    }

    pub fn search(&self, word: &str) -> Option<u32> {
        let mut node = &self.root;
        for char in word.chars() {
            if let Some(next_node) = node.children.get(&char) {
                node = next_node
            } else {
                return None;
            }
        }

        match node.is_leaf {
            true => Some(node.value),
            false => None,
        }
    }
}

fn p2(data: &str) -> Vec<u32> {
        let mut trie = Trie::new();
        for word in vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ] {
            trie.insert(word)
        }
    
        data.split("\n").filter_map(|line| {
            let mut node = &trie.root;
            let mut last_char;
            let res = line.chars().filter_map(|char| {
                if let Some(num) = char.to_digit(10) {
                    // char is already a numner so we can return and reset the search string
                    node = &trie.root;
                    return Some(num);
                } else {
                    // char is _not_ a number so we need to start accumulating and traversing the trie
                    if let Some(next_node) = node.children.get(&char) {
                        // found the char from trie, so we'll use that in the next round
                        node = next_node;
                    } else if  {
                        // Couldn't find path to current char from current node, so resetting
                    }

                    return Some(0);
                }
            });
    
            return res.collect();
        });

    return vec![0];
}

fn get_str_as_int(digit: &str) -> u32 {
    return match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        &_ => 0,
    };
}

#[cfg(test)]
mod tests {
    use super::p1;
    use super::p2;

    #[test]
    fn p1Test() {
        let test_input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        assert_eq!(p1(test_input), 142)
    }

    #[test]
    fn p2Test() {
        let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let resVec: Vec<u32> = p2(test_input);

        assert_eq!(resVec, vec![29, 83, 13, 24, 42, 14, 76]);
        assert_eq!(resVec.iter().sum::<u32>(), 281)
    }
}
