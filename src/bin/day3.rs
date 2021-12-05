use std::cmp::Ordering;
use std::io;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Clone)]
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
        let max_key = max_in_column(&r, col);
        let min_key = min_in_column(&r, col);
        gamma.push(max_key);
        epsilon.push(min_key);
    }
    return bits_to_num(gamma) * bits_to_num(epsilon);
}

pub fn oxygen_generator(report_input: &Report) -> i32 {
    let mut r = report_input.clone();

    for col in 0..r.columns {
        let max_key = max_in_column(&r, col);
        remove_nonmatching_rows(&mut r, col, max_key);
        if r.data.len() == r.columns {
            break;
        }
    }

    let row = r.data.iter().nth(0).unwrap().0.0;

    let mut number = Vec::new();
    for col in 0..r.columns {
        number.push(*r.data.get(&(row, col)).unwrap());
    }
    return bits_to_num(number);
}


pub fn co2_scrubber(report_input: &Report) -> i32 {
    let mut r = report_input.clone();

    for col in 0..r.columns {
        let max_key = min_in_column(&r, col);
        remove_nonmatching_rows(&mut r, col, max_key);
        if r.data.len() == r.columns {
            break;
        }
    }

    let row = r.data.iter().nth(0).unwrap().0.0;

    let mut number = Vec::new();
    for col in 0..r.columns {
        number.push(*r.data.get(&(row, col)).unwrap());
    }
    return bits_to_num(number);
}

fn remove_nonmatching_rows(r: &mut Report, col: usize, bit: char) {
    for row in 0..r.lines {
        match r.data.get(&(row,col)) {
            Some(elem) => {
                if *elem != bit {
                    for col in 0..r.columns {
                        r.data.remove(&(row, col));
                    }
                }
            },
            None => {},
         }     
    }
    
}

fn max_in_column(r: &Report, col: usize) -> char {
    let mut counts = HashMap::from([('0', 0), ('1', 0)]);

    for row in 0..r.lines {
        match r.data.get(&(row,col)) {
            Some(elem) => {
                let n = counts.entry(*elem).or_insert(0);
                *n += 1;
            },
            None => {},
         }        
    }

    return match counts.get(&'0').unwrap().cmp(counts.get(&'1').unwrap()) {
        Ordering::Less => '1',
        Ordering::Equal => '1',
        Ordering::Greater => '0',
    }
}

// I guess this could be derived from maxInColumn but eh
fn min_in_column(r: &Report, col: usize) -> char {
    let mut counts = HashMap::from([('0', 0), ('1', 0)]);

    for row in 0..r.lines {
        match r.data.get(&(row,col)) {
            Some(elem) => {
                let n = counts.entry(*elem).or_insert(0);
                *n += 1;
            },
            None => {},
         }        
    }
    
    return match counts.get(&'0').unwrap().cmp(counts.get(&'1').unwrap()) {
        Ordering::Less => '0',
        Ordering::Equal => '0',
        Ordering::Greater => '1',
    }
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

pub fn life_support(r: &Report) -> i32 {
    return oxygen_generator(r) * co2_scrubber(r);
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let result = life_support(&input.parse().expect("Failed to parse"));
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

    #[test]
    fn test_life_support() {
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

        assert_eq!(oxygen_generator(&report.parse().unwrap()), 23);
        assert_eq!(co2_scrubber(&report.parse().unwrap()), 10);
        assert_eq!(life_support(&report.parse().unwrap()), 230);
    }

}