use crate::{Solution, SolutionPair};
use std::{collections::HashMap, env, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let path_to_input = "input/01.txt";
    let full_path = env::current_dir().unwrap().join(path_to_input);
    println!("path: {}", full_path.to_string_lossy());
    let input = read_to_string(full_path).unwrap();
    let sol1: u32 = p1(&input);
    let sol2: u32 = 0;

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

#[derive(Default, Debug)]
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
}

fn p2(data: &str) -> Vec<Vec<u32>> {
        let mut trie = Trie::new();
        for word in vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ] {
            trie.insert(word)
        }
    
        return data.split("\n").
            filter(|l| !l.is_empty())
            .filter_map(|line| {
                let root_children = &trie.root.children;
                let mut node = &trie.root;
                let mut last_char = 'x';
                println!("Taking in line: {}", line);
                let res = line.chars().filter_map(|char| {
                    print!("{} -> ", char);
                    if let Some(num) = char.to_digit(10) {
                        // char is already a number so we can return and reset the search string
                        node = &trie.root;
                        println!("Is a number! Resetting node to root");
                        return Some(num);
                    }

                    // char is _not_ a number so we need to start traversing the trie
                    if let Some(next_node) = node.children.get(&char) {
                        // found the char from trie, so we'll use that in the next round
                        print!("found next node: ");
                        node = next_node;
                        last_char = char;
                        return if node.is_leaf {
                            // The node we found is a leaf node and thus should have a value
                            // assosiated with it
                            node = &trie.root;
                            println!("Found a match! {}. Resetting to root node", node.value);
                            Some(node.value)
                        } else {
                            None
                        }
                    }  else {
                        // Char was not a direct child of the current node
                        println!("No match in node for {}", char);
                        node = match root_children.get(&last_char) {
                            // Last letter was the beginning of a new word
                            // This should take care of eightwo situtation...
                            Some(found_node) => found_node.children.get(&char).or(Some(&trie.root)).unwrap(),
                            None => &trie.root
                        };
                    }
                    return None;
                }).collect::<Vec<u32>>();
            println!("Constructed {:?}", res);
            Some(res)
        }).collect();
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
    use super::Trie;

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

        //let resVec: Vec<Vec<u32>> = p2(test_input);

        //assert_eq!(resVec, vec![29, 83, 13, 24, 42, 14, 76]);
        //assert_eq!(resVec.iter().sum::<u32>(), 281)
    }

    #[test]
    fn p2Test2() {
        let test_input = "1eightwo";
        let res: Vec<Vec<u32>> = p2(test_input);

        //assert_eq!(res, vec![vec![1,2]])
    }

    #[test]
    fn trieTest() {
        let mut trie = Trie::new();
        for word in vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ] {
            trie.insert(word)
        }

        match trie.root.children.get(&'n') {
            Some(v) => println!("FOUND 'n' {:?}", v ),
            None => println!("Trie not working")
        };

    }
}
