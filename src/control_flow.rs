/*
 * File: control_flow.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::time::Instant;

use colored::{ColoredString, Colorize};

pub(crate) fn control_flow() {
    println!("\nBEGIN CONTROL FLOW\n");
    println!("Please input a number!");
    let number = validate_input_str_to_int();
    println!("{} {}", "Your number is:".green(), number);
    let instant_before_execution = Instant::now();
    less_than_or_equal_to_seven(number);
    let less_than_or_equal_to_seven = time(
        instant_before_execution,
        "less_than_or_equal_to_seven(number)",
    );
    let instant_before_execution = Instant::now();
    equal_to_ten(number);
    let equal_to_ten = time(instant_before_execution, "equal_to_ten(number)");
    let instant_before_execution = Instant::now();
    loop_count_towards(number);
    let loop_count_towards = time(instant_before_execution, "loop_count_towards(number)");
    let instant_before_execution = Instant::now();
    for_range_count_towards(number);
    let for_range_count_towards = time(instant_before_execution, "for_range_count_towards(number)");
    let instant_before_execution = Instant::now();
    while_count_down(number);
    let while_count_down = time(instant_before_execution, "while_count_down(number)");
    println!("\nBEGIN CONTROL FLOW - COLLECTIONS\n");
    let instant_before_execution = Instant::now();
    for_collection();
    let for_collection = time(instant_before_execution, "for_collection()");
    println!("\nBEGIN CONTROL FLOW - TIMINGS\n");
    println!(
        "{} {}",
        less_than_or_equal_to_seven.0, less_than_or_equal_to_seven.1
    );
    println!("{} {}", equal_to_ten.0, equal_to_ten.1);
    println!("{} {}", loop_count_towards.0, loop_count_towards.1);
    println!(
        "{} {}",
        for_range_count_towards.0, for_range_count_towards.1
    );
    println!("{} {}", while_count_down.0, while_count_down.1);
    println!("{} {}", for_collection.0, for_collection.1);
}

fn for_collection() {
    let collection = [-0.213, 846.324231, 89.90098734, -999.7363];
    for element in collection.iter() {
        println!("The value is: {}", element);
    }
}

fn while_count_down(number: i32) {
    println!("{}", "Now we will count the number down to 0!".cyan());
    let mut temp_number = number;
    while temp_number >= 0 {
        println!("{} {}", "3rd>".cyan(), temp_number);
        temp_number -= 1;
    }
    println!("{}", "LIFTOFF!".green());
}

fn for_range_count_towards(number: i32) {
    println!(
        "{}",
        "Now we will count until the number has been reached again!".cyan()
    );
    for num in 0..number + 1 {
        println!("{} {}", "2nd>".cyan(), num);
    }
    println!("{}", "The number has been reached again!".green());
}

fn loop_count_towards(number: i32) {
    println!(
        "{}",
        "Now we will count until the number has been reached!".cyan()
    );
    let mut counter = 0;
    println!("{} {}", "1st>".cyan(), counter);
    loop {
        if counter >= number.abs() {
            println!("{}", "The number has been reached!".green());
            break;
        }
        counter += 1;
        println!("{} {}", "1st>".cyan(), counter);
    }
}

#[allow(clippy::needless_bool)]
fn equal_to_ten(number: i32) {
    let result = if number == 10 { true } else { false };
    if result {
        println!("{}", "The number is 10!".green());
    } else {
        println!("{}", "The number is not 10!".yellow());
    }
}

#[allow(clippy::comparison_chain)]
fn less_than_or_equal_to_seven(number: i32) {
    if number < 7 {
        println!("{}", "The number is smaller than 7!".green());
    } else if number == 7 {
        println!("{}", "The number is equal to 7!".green());
    } else {
        println!("{}", "None of the conditions were met!".yellow());
    }
}

fn validate_input_str_to_int() -> i32 {
    use std::io;
    let number: i32;
    loop {
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(string) => string,
            Err(_) => {
                println!("{}", "WARNING: Failed to read line!".bright_red());
                continue;
            }
        };
        let guess = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "WARNING: Please type a number!".bright_red());
                continue;
            }
        };
        number = guess;
        break;
    }
    number
}

fn time(instant: Instant, function: &str) -> (ColoredString, u128) {
    let mut output = function.to_string();
    output.push_str("> : ");
    (
        output.trim().bright_magenta(),
        instant.elapsed().as_micros(),
    )
}
