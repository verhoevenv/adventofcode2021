use std::io;
use std::io::Read;

type Score = u64;

enum Either<L,R> {
    Left(L),
    Right(R),
}

fn opening_pair(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!(),
    }
}

fn score_illegal(c: char) -> Score {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}


fn score_incomplete(stack: Vec<char>) -> Score {
    let mut score = 0;

    for c in stack.iter().rev() {
        score *= 5;
        score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!(),
        };
    }
    
    return score;
}


fn find_illegal_character(s: &str) -> Either<char, Vec<char>> {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let top = stack.pop().unwrap();
                if top != opening_pair(c) {
                    return Either::Left(c);
                }
            },
            _ => panic!(),
        }
    }

    return Either::Right(stack);
}

pub fn score_illegal_character(s: &str) -> Score {
    s.lines()
     .map(|l| find_illegal_character(l))
     .filter_map(|r| match r {
        Either::Left(c) => Some(c),
        Either::Right(_) => None,
     })
     .map(|c| score_illegal(c))
     .sum()
}

pub fn score_incomplete_lines(s: &str) -> Score {
    let mut score: Vec<_> = s.lines()
     .map(|l| find_illegal_character(l))
     .filter_map(|r| match r {
        Either::Left(_) => None,
        Either::Right(stack) => Some(stack),
     })
     .map(|stack| score_incomplete(stack))
     .collect();

     score.sort();

     return score[score.len() / 2];
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let result = score_incomplete_lines(&input);
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_illegal_character() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(score_illegal_character(&input), 26397);
        assert_eq!(score_incomplete_lines(&input), 288957);
    }   

}