use std::io;
use std::io::Read;
use std::str::FromStr;

type Course = Vec<Command>;

pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(' ')
                                 .collect();

        let amount_fromstr = coords[1].parse::<i32>().or(Err(()))?;

        return match coords[0] {
            "forward" => Ok(Self::Forward(amount_fromstr)),
            "down" => Ok(Self::Down(amount_fromstr)),
            "up" => Ok(Self::Up(amount_fromstr)),
            _ => Err(())
        }
    }
}

pub fn follow_course(course: Course) -> i32 {
    let mut pos = 0;
    let mut depth = 0;

    for command in course {
        match command {
            Command::Forward(x) => pos += x,
            Command::Down(x) => depth += x,
            Command::Up(x) => depth -= x,
        }
    }

    return pos * depth;
}


pub fn follow_course2(course: Course) -> i32 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in course {
        match command {
            Command::Forward(x) => {
                pos += x;
                depth += aim * x;
            }
            Command::Down(x) => aim += x,
            Command::Up(x) => aim -= x,
        }
    }

    return pos * depth;
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut list: Course = Vec::new();
    for line in input.lines() {
        let number: Command = line.trim().parse().expect("Failed to parse input");
        list.push(number);
    }

    let result = follow_course2(list);
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::Command::*;

    #[test]
    fn test_follow_course() {
        let course = vec![
            Forward(5),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2)];

        assert_eq!(follow_course(course), 150);
    }


    #[test]
    fn test_follow_course2() {
        let course = vec![
            Forward(5),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2)];

        assert_eq!(follow_course2(course), 900);
    }

}