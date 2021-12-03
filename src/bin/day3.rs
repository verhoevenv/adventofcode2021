use std::io;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

pub struct Report {
    lines: usize,
    columns: usize,
    data: HashMap<(usize, usize), char>,
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = HashMap::new();
        let mut lines = 0;
        let mut columns = 0;

        for (row, line) in s.lines().enumerate() {
            columns = line.len();
            for (col, c) in line.chars().enumerate() { 
                res.insert((row, col), c);
            }
            lines += 1;
        }

        return Ok(Report {
            lines,
            columns,
            data: res,
        });
    }
}

pub fn power_consumption(r: Report) -> i32 {
    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();

    for col in 0..r.columns {
        let mut counts = HashMap::from([('0', 0), ('1', 0)]);
        for row in 0..r.lines {
            let elem = r.data.get(&(row,col)).unwrap();
            let n = counts.entry(*elem).or_insert(0);
            *n += 1;
        }
        let max_key = counts.iter().max_by_key(|(_k, v)| *v).unwrap().0;
        let min_key = counts.iter().min_by_key(|(_k, v)| *v).unwrap().0;
        gamma.push(*max_key);
        epsilon.push(*min_key);
    }
    return bits_to_num(gamma) * bits_to_num(epsilon);
}

fn bits_to_num(input: Vec<char>) -> i32 {
    let mut res = 0;
    for c in input {
        match c {
            '0' => res = res * 2,
            '1' => res = res * 2 + 1,
            _ => panic!()
        }
    }
    return res;
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let result = power_consumption(input.parse().expect("Failed to parse"));
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_num() {
        assert_eq!(bits_to_num(vec!['1', '0', '1', '1', '0']), 22);
    }

    #[test]
    fn test_power_consumption() {
        let report =
"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(power_consumption(report.parse().unwrap()), 198);
    }

}