use std::fs::read_to_string;

fn process_input() -> Vec<Vec<i16>> {
    read_to_string("inputs/second/input.txt")
        .expect("Error reading input text")
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|s| s.parse::<i16>().expect("Error parsing char into usize"))
                .collect()
        })
        .collect()
}

/// Determines if the vector of numbers are monotically increasing of decreasing
fn is_monotonic(nums: &[i16]) -> bool {
    let all_increasing = nums
        .iter()
        .enumerate()
        .all(|(i, n)| i == 0 || *n < nums[i - 1]);

    let all_decreasing = nums
        .iter()
        .enumerate()
        .all(|(i, n)| i == 0 || *n > nums[i - 1]);

    all_increasing || all_decreasing
}

// Determines if every adjacent element pair differs by at least 1 and at most 3
fn adjacent_diff(nums: &[i16]) -> bool {
    nums.iter().enumerate().all(|(i, n)| {
        if i == 0 {
            return true;
        }
        let diff = (*n - nums[i - 1]).abs();

        (1..=3).contains(&diff)
    })
}

fn subsets<T: FnMut(&[i16]) -> bool, S: FnMut(&[i16]) -> bool>(
    nums: &[i16],
    mut func1: T,
    mut func2: S,
) -> bool {
    //println!("{nums:?}");
    let mut valids = vec![];
    for i in 0..nums.len() {
        let before_idx = &nums[0..i];
        let after_idx = &nums[i + 1..nums.len()];

        let both = [before_idx, after_idx].concat();

        //println!("{:?}", both);

        valids.push(func1(&both) && func2(&both));
    }

    [valids, vec![func1(nums) && func2(nums)]]
        .concat()
        .iter()
        .any(|e| *e)
}

pub fn find_safe_reports(dampener: bool) -> usize {
    process_input()
        .iter()
        .map(|nums| {
            if dampener {
                subsets(nums, is_monotonic, adjacent_diff)
            } else {
                is_monotonic(nums) && adjacent_diff(nums)
            }
        })
        .map(|n| if n { 1 } else { 0 })
        .sum::<usize>()
}
