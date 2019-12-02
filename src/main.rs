mod day1;
mod day2;

fn main() {
    let answer_1 = day1::solve();
    println!("Day 1:");
    println!("{}", answer_1);
    println!("===============================================");
    println!("Day 2:");
    let answer_2 = day2::solve();
    match answer_2 {
        Some(actual_answer_2) => println!("{}", actual_answer_2),
        None => println!("Cannot find answer for day 2"),
    }
}
