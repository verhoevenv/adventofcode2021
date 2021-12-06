use std::io;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

type Lifetime = u8;
type FishCount = u128;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Population(HashMap<Lifetime, FishCount>);

impl Population {
    fn num_fish(&self) -> FishCount {
        self.0.values().sum()
    }
}

impl FromStr for Population {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for lifetime in s.split(",").map(|n| n.parse().unwrap()) {
            let count = map.entry(lifetime).or_insert(0);
            *count += 1;
        }
        return Ok(Population(map));
    }
}

pub fn step(pop: &Population) -> Population {
    let mut map = HashMap::new();

    for (lifetime, count) in pop.0.iter() {
        match lifetime {
            0 => {
                *map.entry(6).or_insert(0) += count;
                *map.entry(8).or_insert(0) += count;
            },
            x => *map.entry(*x - 1).or_insert(0) += count,
        }
    }
    return Population(map);
}

pub fn simulate(pop: &Population, steps: u32) -> Population {
    // No idea how to get rid of this clone, which seems unnecessary to me :/
    // Probably doing something wrong with lifetimes?
    //   Update: could be solved by
    //    - replace .clone() with calling step once before the loop (& adjust loop to 1..steps)
    //    - changing the design to mutate the population instead of making immutable copies
    let mut current_pop = pop.clone();
    for _ in 0..steps {
        current_pop = step(&current_pop);
    }
    return current_pop;
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let result = simulate(&input.parse().expect("Failed to parse"), 256);
    println!("{}", result.num_fish());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let pop = "3,4,3,1,2".parse().unwrap();
        assert_eq!(step(&pop), "2,3,2,0,1".parse().unwrap());
    }

    #[test]
    fn test_simulate() {
        let pop = "3,4,3,1,2".parse().unwrap();
        assert_eq!(simulate(&pop, 18), "6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8".parse().unwrap());
        assert_eq!(simulate(&pop, 256).num_fish(), 26984457539);
    }
    

}