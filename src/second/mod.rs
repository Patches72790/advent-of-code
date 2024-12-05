use std::fs::read_to_string;

fn process_input() -> Vec<Vec<i16>> {
    let input = read_to_string("inputs/second/input.txt").expect("Error reading input text");

    input
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

fn adjacent_diff(nums: &[i16]) -> bool {
    let adj = nums.iter().enumerate().all(|(i, n)| {
        if i == 0 {
            return true;
        }
        let diff = (*n - nums[i - 1]).abs();

        (1..=3).contains(&diff)
    });

    adj
}

pub fn find_safe_reports() -> usize {
    let input = process_input();

    let monotonic = input
        .iter()
        .map(|nums| {
            let subsets = nums
                .into_iter().map(|n| )

            return is_monotonic(nums);
        })
        .collect::<Vec<bool>>();

    let adjacent_diffs = input
        .iter()
        .map(|nums| adjacent_diff(nums))
        .collect::<Vec<bool>>();

    let safe = monotonic
        .iter()
        .zip(adjacent_diffs.iter())
        .map(|(m, a)| if *m && *a { 1 } else { 0 })
        .sum::<usize>();

    safe
}
