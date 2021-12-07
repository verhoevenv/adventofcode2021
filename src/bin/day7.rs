use std::io;
use std::io::Read;

pub fn min_distance1(nums: Vec<i64>) -> i64 {
    // the median minimizes the sum of euclidian distances
    let median = median(&nums);

    return nums.iter().fold(0, |acc, x| acc + fuel_cost_linear(median, *x));
}

pub fn min_distance2(nums: Vec<i64>) -> i64 {
    // I feel like it should be possible to find a closed form solution,
    // but my maths are rusty, so we'll do it with very naive searching
    let mean = minimize(&nums, fuel_cost_nonlinear);

    return nums.iter().fold(0, |acc, x| acc + fuel_cost_nonlinear(mean, *x));
}

fn fuel_cost_linear(from: i64, to: i64) -> i64 {
    (from - to).abs()
}

fn fuel_cost_nonlinear(from: i64, to: i64) -> i64 {
    (from - to).abs() * ((from - to).abs() + 1) / 2
}


fn median(nums: &Vec<i64>) -> i64 {
    let mut nums = nums.clone();
    nums.sort();
    let n = nums.len();
    if n % 2 == 0 {
        // this isn't mathematically correct but should be good enough,
        // since we assume the solution can't have non-integer positions
        return nums[n/2];
    } else {
        return nums[(n/2)+1];
    }
}

// assuming convex function but no derivative available (since I'm lazy)
fn minimize(nums: &Vec<i64>, cost: fn(i64, i64) -> i64) -> i64 {
    let eval = |pos| nums.iter().fold(0, |acc, x| acc + cost(pos, *x));
    let mut best_pos = median(nums);
    let mut best_guess = eval(best_pos);
    loop {
        //this does way too many calculations but we can improve if needed
        // - take bigger steps (use gradient / binary search)
        // - memoize already evalled positions
        let left = eval(best_pos - 1);
        let right = eval(best_pos + 1);
        if left < best_guess {
            best_guess = left;
            best_pos = best_pos - 1;
        } else if right < best_guess {
            best_guess = right;
            best_pos = best_pos + 1;
        } else {
            return best_pos;
        }
    }
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let nums: Vec<i64> = input.split(",").map(|n| n.parse().unwrap()).collect();

    let result = min_distance2(nums);
    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance1() {
        assert_eq!(min_distance1(vec![16,1,2,0,4,2,7,1,2,14]), 37);
    }   

    #[test]
    fn test_min_distance2() {
        assert_eq!(min_distance2(vec![16,1,2,0,4,2,7,1,2,14]), 168);
    }   
}