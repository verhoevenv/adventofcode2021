use std::str::FromStr;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct XY (Coord, Coord);

type EnergyLevel = u32;
type Coord = i32;

pub struct Grid<T> {
    map: HashMap<XY, T>,
    size: XY,
}

impl<T: FromStr> Grid<T> {
    fn get(&mut self, p: &XY) -> &mut T {
        self.map.get_mut(p).unwrap()
    }

    fn contains(&self, p: &XY) -> bool {
        let XY(x, y) = p;
        let XY(size_x, size_y) = self.size;

        *x >= 0 && *x < size_x && *y >= 0 && *y < size_y
    }

    fn neighbours(&mut self, p: &XY) -> Vec<(XY, &mut T)> {
        let mut neighbours = Vec::new();
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                let XY(x, y) = p;
                let neighbour = XY(x + dx, y + dy);
                if (dx != 0 || dy != 0) && self.contains(&neighbour) {
                    neighbours.push(neighbour);
                }
            }
        }

        /*
        Would prefer 

        neighbours.iter()
                  .map(|xy| (*xy, self.get(xy)))
                  .collect()

        but rust doesn't like this?
        */
        self.map.iter_mut()
                .filter(|(k, _)| neighbours.contains(k))
                .map(|(k, v)| (*k, v))
                .collect()
    }
}

impl<T> FromStr for Grid<T>
    where T: FromStr, T::Err: Debug {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();

        let mut size_x = 0;
        let mut size_y = 0;
        for (x, l) in s.lines().enumerate() {
            size_x += 1;
            size_y = 0;
            for (y, c) in l.chars().enumerate() {
                size_y += 1;
                map.insert(XY(x.try_into().unwrap(), y.try_into().unwrap()), c.to_string().parse().unwrap());
            }
        }

        Ok(Grid {map: map, size: XY(size_x, size_y) })
    }
}

fn step(grid: &mut Grid<EnergyLevel>) -> u32 {
    let XY(size_x, size_y) = grid.size;
    let mut flashes_todo = Vec::new();
    let mut flashes_done = Vec::new();

    for x in 0..size_x {
        for y in 0..size_y {
            let energy = grid.get(&XY(x, y));
            *energy += 1;
            if *energy > 9 {
                flashes_todo.push(XY(x, y));
            }
        }
    }

    while let Some(p) = flashes_todo.pop() {
        for (neighbour_xy, energy) in grid.neighbours(&p) {
            *energy += 1;
            if *energy > 9 &&
                !flashes_done.contains(&neighbour_xy) &&
                !flashes_todo.contains(&neighbour_xy) {
                    flashes_todo.push(neighbour_xy);
            }
        }
        flashes_done.push(p);
    }

    for x in 0..size_x {
        for y in 0..size_y {
            let energy = grid.get(&XY(x, y));
            if *energy > 9 {
                *energy = 0;
            }
        }
    }

    return flashes_done.len().try_into().unwrap();
}

pub fn count_flashes(grid: &mut Grid<EnergyLevel>, steps: u32) -> u32 {
    let mut num_flashes = 0;
    for _ in 0..steps {
        num_flashes += step(grid);
    }
    return num_flashes;
}


pub fn when_synced(grid: &mut Grid<EnergyLevel>) -> u32 {
    let mut steps = 0;
    let XY(size_x, size_y) = grid.size;
    let num_octopi = (size_x * size_y).try_into().unwrap();
    loop {
        steps += 1;
        let num_flashes = step(grid);
        if num_flashes == num_octopi {
            return steps;
        }
    }
}


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let result = when_synced(&mut input.parse().unwrap());
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_flashes() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        
        assert_eq!(count_flashes(&mut input.parse().unwrap(), 10), 204);
        assert_eq!(count_flashes(&mut input.parse().unwrap(), 100), 1656);
    }   

    #[test]
    fn test_when_synced() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        
        assert_eq!(when_synced(&mut input.parse().unwrap()), 195);
    }   
}