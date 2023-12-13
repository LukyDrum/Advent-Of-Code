use std::time::Instant;

use day::Day;

mod day;
mod days;

use days::*;

fn main() {
    println!("\n* Welcome to Advent Of Code 2023! *\n");

    // Vector of all days
    let mut all_days: Vec<Box<dyn Day>> = vec![
        Day1::new_box(),
        Day2::new_box(),
        Day3::new_box(),
        Day4::new_box(),
        Day5::new_box(),
        Day6::new_box(),
        Day7::new_box(),
        Day8::new_box(),
    ];

    // This will print the output in a table
    println!("┏━━━━━━━┳━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━┓");
    println!("┃ Day \t┃ Part 1 \t┃ Part 2 \t ┃ Time \t ┃");
    println!("┡━━━━━━━╇━━━━━━━━━━━━━━━╇━━━━━━━━━━━━━━━━╇━━━━━━━━━━━━━━━┩");

    let mut total_time = 0;
    let mut i = 1;
    for d in &mut all_days {
        print!("│ {}\t│ ", i);
        let before_day = Instant::now();
        d.setup();
        let part1 = d.part1();
        let part2 = d.part2();

        print!("\t{}\t│\t{}\t │\t", part1, part2);
        let time = before_day.elapsed().as_millis();
        println!("{} ms\t │", time);

        total_time += time;

        i += 1;
    }

    println!("├───────┴───────────────┴────────────────┼───────────────┤");
    println!("│\t\t\t\t\t │\t{} ms\t │", total_time);
    println!("└────────────────────────────────────────┴───────────────┘\n");
}
