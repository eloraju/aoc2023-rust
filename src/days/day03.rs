use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, u64};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &str) -> u64 {
    return 0;
}

#[derive(Debug, PartialEq, Eq)]
enum NodeType {
    Symbol,
    Number(u32),
    Empty,
    Error,
}

#[derive(Debug)]
struct Node {
    node_type: NodeType,
    index: usize,
}

impl Node {
    fn is_digit(&self) -> bool {
        match self.node_type {
            NodeType::Number(_) => true,
            _ => false,
        }
    }

    fn is_symbol(&self) -> bool {
        self.node_type == NodeType::Symbol
    }

    fn is_empty(&self) -> bool {
        self.node_type == NodeType::Empty
    }
}
struct NodeArray {
    nodes: Vec<Node>,
    row_length: usize,
}

impl NodeArray {
    pub fn new(input: &str) -> Self {
        let split = input.split("\n").collect::<Vec<&str>>();
        // Assuming that there's input on the first line :shrug:
        let row_length = split.get(0).unwrap_or(&"").len().try_into().unwrap_or(0);
        let nodes = input
            .split("\n")
            .enumerate()
            .flat_map(|(y, line)| {
                // The "move" here makes sure that the closure takes ownership of the y-variable
                // thus making sure that we don't just try to pass a pointer to y in the return,
                // since the pointer would become a dangling pointer. The y-variable is in local
                // scope and will cause the all pointers to it to become dangling since y will be
                // removed from the stack once we leave the function (flat_map) scope
                return line.chars().enumerate().map(move |(x, c)| match c {
                    '.' => Node {
                        node_type: NodeType::Empty,
                        index: Self::index_from_coords(x, y, row_length),
                    },
                    c if c.is_numeric() => Node {
                        node_type: NodeType::Number(c.to_digit(10).unwrap()),
                        index: Self::index_from_coords(x, y,row_length),
                    },
                    _ => Node {
                        node_type: NodeType::Symbol,
                        index: Self::index_from_coords(x, y, row_length),
                    },
                });
            })
            .collect();
        Self { nodes, row_length }
    }

    pub fn get_neighbours(&self, index: usize) -> Vec<Option<&Node>> {
        let mut neighbours: Vec<Option<&Node>> = Vec::new();
        neighbours.push(self.nodes.get(index - self.row_length - 1));    //nw
        neighbours.push(self.nodes.get(index - self.row_length));        //n
        neighbours.push(self.nodes.get(index - self.row_length + 1));    //ne
        neighbours.push(self.nodes.get(index - 1));                      //e
        neighbours.push(self.nodes.get(index + self.row_length + 1));    //se
        neighbours.push(self.nodes.get(index + self.row_length));        //s
        neighbours.push(self.nodes.get(index + self.row_length - 1));    //sw
        neighbours.push(self.nodes.get(index + 1));                      //w

        return neighbours;
    }

    pub fn get_symbols(&self) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|n| n.node_type == NodeType::Symbol)
            .collect()
    }

    pub fn index_from_coords(x: usize, y: usize, row_length: usize) -> usize {
        return y * self.row_length + x;
    }

    pub fn get_number_from_index(&self, index: usize) -> u32{
        // Node is in the middle or the last value
        let mut node = self.nodes.get(index).unwrap();

    }
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
