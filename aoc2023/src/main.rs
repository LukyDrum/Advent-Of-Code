use day::Day;

mod day;
mod days;

use days::*;

fn main() {
    println!("\nWelcome to Advent Of Code 2023!\n");

    // Vector of all days
    let all_days: Vec<Box<dyn Day>> = vec![Day1::new_box(), Day2::new_box(), Day3::new_box(), Day4::new_box()];

    // This will print the output in a table
    println!("Day\t Part 1 \t Part 2");
    println!("__________________________________________");
    let mut i = 1;
    for d in all_days {
        print!("Day {}\t| ", i);
        println!("\t{}\t|\t{}", d.part1(), d.part2());

        i += 1;
    }

    println!("__________________________________________\n");
}
