use crate::{Solution, SolutionPair};
use std::{env, fs::read_to_string};

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
            return Some(numbers.first().unwrap() * 10 + numbers.last().unwrap());
        })
        .sum();
}

const HR_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn p2(data: &str) -> Vec<u32> {
    let res = data.split("\n").filter(|l| !l.is_empty()).map(|line| {
         let parsed = line.char_indices().filter_map(|(ci, char)| {
            if let Some(val) = char.to_digit(10) {
                return Some(val);
            }

            for (wi, word) in (1u32..).zip(HR_DIGITS.iter()) {
                if line[ci..].starts_with(word) {
                    return Some(wi);
                }
            }
            None
        }).collect::<Vec<u32>>();

         println!("Line: {} - Parsed {:?}", line, parsed);
        return parsed.first().unwrap() * 10 + parsed.last().unwrap();
    });

    return res.collect();
}

#[cfg(test)]
mod tests {
    use super::p1;
    use super::p2;

    #[test]
    fn p1_test() {
        let test_input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        assert_eq!(p1(test_input), 142)
    }

    #[test]
    fn p2_test() {
        let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let test = p2(test_input);
        assert_eq!(test, vec![29, 83, 13, 24, 42, 14, 76]);
        assert_eq!(test.iter().sum::<u32>(), 281)
    }
}
