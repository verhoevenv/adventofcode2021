use regex::Regex;
use std::io;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

type Coord = i32;

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub struct XY {
    x: Coord,
    y: Coord,
}

pub struct VentLines(Vec<VentLine>);

impl FromStr for VentLines {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(VentLines(s.lines().map(|l| l.parse().unwrap()).collect()))
    }
}

#[derive(Debug)]
pub struct VentLine {
    from: XY,
    to: XY,
}

impl VentLine {
    fn points_on_line(&self, with_diagonals: bool) -> Vec<XY> {
        let step_x = (self.to.x - self.from.x).signum();
        let step_y = (self.to.y - self.from.y).signum();

        if step_x != 0 && step_y != 0 && !with_diagonals {
            return Vec::new();
        }
        
        let mut points = Vec::new();
        let mut on_line = self.from;

        while on_line.x != self.to.x || on_line.y != self.to.y {
            points.push(on_line);
            on_line.x += step_x;
            on_line.y += step_y;
        }
        points.push(on_line);

        return points;
    }
}

impl FromStr for VentLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

        let caps = re.captures(s).unwrap();

        let from_x = &caps[1].parse().unwrap();
        let from_y = &caps[2].parse().unwrap();
        let to_x = &caps[3].parse().unwrap();
        let to_y = &caps[4].parse().unwrap();


        return Ok(VentLine {
            from: XY {x: *from_x, y: *from_y},
            to: XY {x: *to_x, y: *to_y},
        });
    }
}

pub fn how_many_overlaps(lines: VentLines, with_diagonals: bool) -> u64 {
    let mut field: HashMap<XY, u64> = HashMap::new();
    for line in lines.0.iter() {
        for p in line.points_on_line(with_diagonals) {
            let count = field.entry(p).or_insert(0);
            *count += 1;
        }
    }

    let mut count = 0;

    for (_, v) in field.iter() {
        if *v >= 2 {
            count += 1;
        }
    }

    return count;
}


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let result = how_many_overlaps(input.parse().expect("Failed to parse"), true);
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_line() {
        let line = VentLine{from: XY{x: 1, y: 1}, to: XY{x: 1, y: 3}};
        assert_eq!(line.points_on_line(false),
                    vec![XY { x: 1, y: 1 }, XY { x: 1, y: 2 }, XY { x: 1, y: 3 }]);
    }

    #[test]
    fn test_on_line_diag() {
        let line = VentLine{from: XY{x: 1, y: 1}, to: XY{x: 3, y: 3}};
        assert_eq!(line.points_on_line(true),
                    vec![XY { x: 1, y: 1 }, XY { x: 2, y: 2 }, XY { x: 3, y: 3 }]);
    }

    #[test]
    fn test_overlaps() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(how_many_overlaps(input.parse().unwrap(), false), 5);
        assert_eq!(how_many_overlaps(input.parse().unwrap(), true), 12);
    }


}