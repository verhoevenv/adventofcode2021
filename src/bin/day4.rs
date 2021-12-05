use std::io;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug,Default)]
pub struct BoardSquare {
    num: u64,
    marked: bool,
}

#[derive(Debug)]
pub struct Board ([[BoardSquare; 5]; 5]);

impl Board {
    fn mark(&mut self, num: u64) {
        for row in 0..5 {
            for col in 0..5 {
                if self.0[row][col].num == num {
                    self.0[row][col].marked = true;
                }
            }
        }
    }

    fn wins(&self) -> bool {
        for row in 0..5 {
            let mut win = true;
            for col in 0..5 {
                if !self.0[row][col].marked {
                    win = false;
                }
            }
            if win {
                return true;
            }
        }

        for col in 0..5 {
            let mut win = true;
            for row in 0..5 {
                if !self.0[row][col].marked {
                    win = false;
                }
            }
            if win {
                return true;
            }
        }

        return false;
    }

    fn sum_unmarked(&self) -> u64 {
        let mut sum = 0;
        for col in 0..5 {
            for row in 0..5 {
                if !self.0[row][col].marked {
                    sum += self.0[row][col].num;
                }
            }
        }
        return sum;
    }
}

#[derive(Debug)]
pub struct Game {
    numbers: Vec<u64>,
    boards: Vec<Board>,
}


impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        
        let numbers: Vec<u64> = lines[0]
                                .split(",")
                                .map(|x| x.parse().unwrap())
                                .collect();
        
        let mut boards = Vec::new();
        for board in lines[1..].chunks(6) {
            let mut b: [[BoardSquare; 5]; 5] = Default::default();

            for (x, nums) in board[1..].iter().enumerate() {
                for (y, num) in nums.split_whitespace().enumerate() {
                    let n = num.parse().unwrap();
                    b[x][y] = BoardSquare{num: n, marked: false}
                }
            }
            boards.push(Board(b));
        }

        return Ok(Game {
            numbers,
            boards,
        });
    }
}

pub fn bingo_first(mut g: Game) -> u64 {
    for n in g.numbers {
        for b in &mut g.boards {
            b.mark(n);
            if b.wins() {
                return b.sum_unmarked() * n;
            }
        }
    }
    panic!("Should have finished!");
}

pub fn bingo_last(mut g: Game) -> u64 {
    for n in g.numbers {       
        for b in &mut g.boards {
            b.mark(n);
        }
        if g.boards.len() == 1 {
            return g.boards[0].sum_unmarked() * n;
        }
        g.boards.retain(|b| !b.wins());
    }
    panic!("Should have finished!");
}


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let result = bingo_last(input.parse().expect("Failed to parse"));
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";
        assert_eq!(bingo_first(input.parse().unwrap()), 4512);
        assert_eq!(bingo_last(input.parse().unwrap()), 1924);
    }


}