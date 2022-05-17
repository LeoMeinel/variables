/*
 * variables is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/TamrielNetwork/project/blob/main/LICENSE
 */
use std::io;

use colored::Colorize;

pub(crate) fn control_flow() {
	println!("\nBEGIN CONTROL FLOW\n");
	println!("Please input a number!");
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
	println!("{} {}", "Your number is:".green(), number);
	if number < 7 {
		println!("{}", "The number is smaller than 7!".green());
	} else if number == 7 {
		println!("{}", "The number is equal to 7!".green());
	} else {
		println!("{}", "None of the conditions were met!".yellow());
	}
	let result = if number == 10 { true } else { false };
	if result {
		println!("{}", "The number is 10!".green());
	} else {
		println!("{}", "The number is not 10!".yellow());
	}
	println!("{}", "Now we will count until the number has been reached!".cyan());
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
	println!("{}", "Now we will count until the number has been reached again!".cyan());
	for num in 0..number + 1 {
		println!("{} {}", "2nd>".cyan(), num);
	}
	println!("{}", "The number has been reached again!".green());
	println!("{}", "Now we will count the number down to 0!".cyan());
	let mut temp_number = number;
	while temp_number >= 0 {
		println!("{} {}", "3rd>".cyan(), temp_number);
		temp_number -= 1;
	}
	println!("{}", "LIFTOFF!".green());
	println!("\nBEGIN CONTROL FLOW - COLLECTIONS\n");
	let collection = [counter, number, temp_number];
	for element in collection.iter() {
		println!("The value is: {}", element);
	}
}