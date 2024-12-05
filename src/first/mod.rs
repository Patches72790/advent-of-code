use std::fs::read_to_string;

fn get_sorted_list_pair() -> (Vec<usize>, Vec<usize>) {
    let input = read_to_string("inputs/first/part1.txt").expect("Error getting input");

    let mut first = vec![];
    let mut second = vec![];

    for pairs in input.lines() {
        let sp = pairs
            .split("   ")
            .map(String::from)
            .collect::<Vec<String>>();

        first.push(sp[0].parse::<usize>().expect("Error parsing num1"));
        second.push(sp[1].parse::<usize>().expect("Error parsing num2"));
    }

    first.sort();
    second.sort();

    return (first, second);
}

pub fn list_diff() {
    let (first, second) = get_sorted_list_pair();

    let sum: usize = first
        .iter()
        .zip(second.iter())
        .map(|(f, s)| f.abs_diff(*s))
        .sum();

    println!("List difference: {}", sum);
}

pub fn list_similarity() {
    let (first, second) = get_sorted_list_pair();

    let sum: usize = first
        .iter()
        .map(|a| {
            let count = second.iter().filter(|n| *n == a).count();

            return a * count;
        })
        .sum();

    println!("Similarity score: {}", sum);
}
