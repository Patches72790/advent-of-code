use aoc_2024::{
    first::{list_diff, list_similarity},
    second::find_safe_reports,
};

fn main() {
    // day1
    //list_diff();
    //list_similarity();

    // day2
    println!("Safe reports: {}", find_safe_reports(false));
    println!("Safe reports (dampening of 1): {}", find_safe_reports(true));
}
