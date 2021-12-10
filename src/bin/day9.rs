use std::str::FromStr;
use std::collections::HashMap;
use std::io;
use std::io::Read;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct XY (Coord, Coord);

type Height = u32;
type Coord = usize;

pub struct Heightmap {
    map: HashMap<XY, Height>,
    size: XY,
}

impl Heightmap {
    fn adjacents(&self, p: &XY) -> Vec<(XY, Height)> {
        let mut adjacents = Vec::new();

        match p {
            XY(0, y) => adjacents.push(XY(1, *y)),
            XY(x, y) if *x == (self.size.0 - 1) => adjacents.push(XY(x - 1, *y)),
            XY(x, y) => adjacents.extend([XY(x - 1, *y), XY(x + 1, *y)].iter()),
        }

        match p {
            XY(x, 0) => adjacents.push(XY(*x, 1)),
            XY(x, y) if *y == (self.size.1 - 1) => adjacents.push(XY(*x, *y - 1)),
            XY(x, y) => adjacents.extend([XY(*x, *y - 1), XY(*x, *y + 1)].iter()),
        }

        adjacents.iter().map(|xy| (*xy, *self.map.get(xy).unwrap())).collect() 
    }
}

impl FromStr for Heightmap {
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
                map.insert(XY(x, y), c.to_digit(10).unwrap());
            }
        }

        Ok(Heightmap {map: map, size: XY(size_x, size_y) })
    }
}

fn lowpoints(map: &Heightmap) -> Vec<(XY, Height)> {
    let XY(size_x, size_y) = map.size;
    let mut result = Vec::new();
    for x in 0..size_x {
        for y in 0..size_y {
            let xy = XY(x,y);
            let h = map.map.get(&xy).unwrap();
            if map.adjacents(&xy).iter().all(|(_, ah)| ah > h) {
                result.push((xy, *h));
            }

        }
    }
    return result;
}

pub fn sum_risks_lowpoints(map: &Heightmap) -> u32 {
    lowpoints(map).iter().map(|p| p.1 + 1).sum()
}


fn find_basin(map: &Heightmap, low_point: &XY) -> Vec<XY> {
    let mut to_expand = vec![*low_point];
    let mut basin = vec![*low_point];
    while let Some(p) = to_expand.pop() {
        for (neighbour_xy, neighbour_h) in map.adjacents(&p) {
            if neighbour_h < 9 && !basin.contains(&neighbour_xy) {
                to_expand.push(neighbour_xy);
                basin.push(neighbour_xy);
            }
        }
    }

    return basin;
}

pub fn largest_basins(map: &Heightmap) -> usize {
    let mut basins: Vec<_> = lowpoints(map).iter()
                                           .map(|(xy, _)| find_basin(map, xy))
                                           .map(|b| b.len())
                                           .collect();
    basins.sort();
    basins.reverse();
    return basins.iter().take(3).product();
}


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let result = largest_basins(&input.parse().unwrap());
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_risks_lowpoints() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(sum_risks_lowpoints(&input.parse().unwrap()), 15);
    }   

    #[test]
    fn test_largest_basins() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(largest_basins(&input.parse().unwrap()), 1134);
    }   
}