use std::io;
use std::io::Read;

pub fn num_increasing(list: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut previous = &list[0];

    for current in &list[1..] {
        if current > previous {
            count += 1;
        }
        previous = current;
    }

    return count;
}

pub fn num_increasing_window(list: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut previous = &list[0] + &list[1] + &list[2];
    let l = list.len();

    for (i, val) in list[0..l-3].iter().enumerate() {
        let current = previous - val + &list[i+3];
        if current > previous {
            count += 1;
        }
        previous = current;
    }

    return count;
}

fn main() {
    
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut list: Vec<i32> = Vec::new();
    for line in input.lines() {
        let number: i32 = line.trim().parse().expect("Failed to parse input");
        list.push(number);
    }

    let result = num_increasing_window(list);
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_increasing() {
        let vals = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(num_increasing(vals), 7);
    }

    #[test]
    fn test_num_increasing_window() {
        let vals = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(num_increasing_window(vals), 5);
    }
}