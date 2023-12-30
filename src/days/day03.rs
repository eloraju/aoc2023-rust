use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, u64, collections::HashMap, fmt::Display};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &str) -> u64 {
    return create_grid(input);
}


#[derive(Debug, Default)]
struct Node {
    is_empty: bool,
    // If the node is not empty and the value of number is None, we have a symbol
    number: Option<u32>,
}

impl Node {
    pub fn is_symbol(&self) -> bool {
        return self.is_empty && self.number.is_none()
    }
}

#[derive(Debug, Default)]
struct Grid {
    grid: HashMap<(usize,usize), Node>
}

impl Grid {
    pub fn add(&mut self, cords: (usize,usize), node: Node) {
        self.grid.insert(cords, node);
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn create_grid(input: &str) -> Grid {
    let mut grid = Grid::default();
    input.split("\n").enumerate().for_each(|(col_index,line)| line.chars().enumerate().for_each(|(row_index,c)| {
        let node = Node {
            is_empty: c == '.',
            number: c.to_digit(10),
        };
        grid.add((row_index, col_index),node);
    }));

    return grid;
}

fn create_vec(input: &str) -> Vec<char> {
    input.split("\n").flat_map(|line| line.chars()).collect()

}



#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"
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
";

    const TEST_INPUT_2: &str = r"";

    #[test]
    fn p1_test() {
        let result = part1(TEST_INPUT_1);

        assert_eq!(result, 4361);
    }

    #[test]
    fn p2_test() {}
}
