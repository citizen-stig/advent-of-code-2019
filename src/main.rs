mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    println!("Day 1:");
    let answer_1 = day1::solve();
    println!("{}", answer_1);

    println!("===============================================");
    println!("Day 2:");
    let answer_2 = day2::solve();
    match answer_2 {
        Some(actual_answer_2) => println!("{}", actual_answer_2),
        None => println!("Cannot find answer for day 2"),
    }

//    println!("===============================================");
//    println!("Day 3:");
//    let answer_3 = day3::solve();
//    println!("{}", answer_3);

    println!("===============================================");
    println!("Day 4:");
    let answer_4 = day4::solve();
    println!("{}", answer_4)

}
