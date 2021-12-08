#[macro_use]
extern crate lazy_static;

use std::str::FromStr;
use std::collections::HashSet;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use Segment::*;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Segment {
    A, B, C, D, E, F, G
}

static SEGMENTS: [Segment; 7] = [
    A, B, C, D, E, F, G
];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Digit {
    One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero
}

lazy_static! {
    static ref DIGIT_SEGMENTS: HashMap<Digit, HashSet<&'static Segment>> = {
        let mut m = HashMap::new();
        m.insert(Digit::One, HashSet::from([&C, &F]));
        m.insert(Digit::Two, HashSet::from([&A, &C, &D, &E, &G]));
        m.insert(Digit::Three, HashSet::from([&A, &C, &D, &F, &G]));
        m.insert(Digit::Four, HashSet::from([&B, &C, &D, &F]));
        m.insert(Digit::Five, HashSet::from([&A, &B, &D, &F, &G]));
        m.insert(Digit::Six, HashSet::from([&A, &B, &D, &E, &F, &G]));
        m.insert(Digit::Seven, HashSet::from([&A, &C, &F]));
        m.insert(Digit::Eight, HashSet::from([&A, &B, &C, &D, &E, &F, &G]));
        m.insert(Digit::Nine, HashSet::from([&A, &B, &C, &D, &F, &G]));
        m.insert(Digit::Zero, HashSet::from([&A, &B, &C, &E, &F, &G]));
        m
    };
}

impl Digit {
    fn segments(&self) -> &HashSet<&Segment> {
        &DIGIT_SEGMENTS.get(self).unwrap()
    }

    fn val(&self) -> i32 {
        match self {
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
            Digit::Zero => 0,
        }
    }
    
    fn as_digit(signals: &Pattern) -> Option<Digit> {
        for d in &DIGITS {
            if &signals.0 == d.segments() {
                return Some(*d);
            }
        }
        return None;
    }
}


static DIGITS: [Digit; 10] = [
    Digit::Zero,
    Digit::One,
    Digit::Two,
    Digit::Three,
    Digit::Four,
    Digit::Five,
    Digit::Six,
    Digit::Seven,
    Digit::Eight,
    Digit::Nine
];

struct Pattern(HashSet<&'static Segment>);

impl Pattern {
    fn mapped(&self, mapping: &Mapping) -> Pattern {
        let mapped: HashSet<&Segment> = self.0.iter()
                                             .map(|seg| *mapping.0.get(seg).unwrap())
                                             .collect();
        Pattern(mapped)
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = HashSet::new();

        for c in s.chars() {
            let s = match c {
                'a' => &A,
                'b' => &B,
                'c' => &C,
                'd' => &D,
                'e' => &E,
                'f' => &F,
                'g' => &G,
                _ => panic!("unknown character"),
            };
            result.insert(s);
        }

        return Ok(Pattern(result));
    }
}

pub struct Entry {
    patterns: Vec<Pattern>,
    output: Vec<Pattern>
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x: Vec<Vec<&str>> = s.split(" | ")
         .map(|p| p.split_whitespace().collect())
         .collect();

         let patterns = x[0].iter().map(|p| p.parse().unwrap()).collect();
         let output = x[1].iter().map(|p| p.parse().unwrap()).collect();
         Ok(Entry{patterns, output})
    }
}

struct Mapping(HashMap<&'static Segment, &'static Segment>);

fn test_mapping(signals: &Vec<Pattern>, mapping: &Mapping) -> bool {
    for signal in signals {
        let mapped = signal.mapped(mapping);
        if Digit::as_digit(&mapped).is_none() {
            return false;
        }

    }
    return true;
}

fn find_mapping(p: &Vec<Pattern>) -> Mapping {
    SEGMENTS.iter()
           .permutations(7)
           .map(|l| -> Mapping {
                let x = SEGMENTS.iter().zip(l);
                return Mapping(HashMap::from_iter(x));
            })
           .find(|m| test_mapping(p, m))
           .unwrap()
}

pub fn decode(entry: &Entry) -> i32 {
    let correct_map = find_mapping(&entry.patterns);
    let mut result = 0;
    for digit_displayed in &entry.output {
        result *= 10;
        let mapped_digit = digit_displayed.mapped(&correct_map);
        let val = Digit::as_digit(&mapped_digit).unwrap().val();
        result += val;
    }
    return result;
}

pub fn digits_1478_in_output(entries: &Vec<Entry>) -> i32 {
    let mut count = 0;
    for entry in entries {
        for digit in &entry.output {
            match digit.0.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => {}
            }
        }
    }

    return count;
}


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let entries: Vec<Entry> = input.lines().map(|n| n.parse().unwrap()).collect();

    let result: i32 = entries.iter().map(|e| decode(&e)).sum();
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits_1478_in_output() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        
        let entries: Vec<Entry> = input.lines().map(|n| n.parse().unwrap()).collect();

        assert_eq!(digits_1478_in_output(&entries), 26);
    }   

    #[test]
    fn test_find_mapping() {
        let entry = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let entry: Entry = entry.parse().unwrap();
        assert_eq!(decode(&entry), 5353);
    }
}