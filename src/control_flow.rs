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

use colored::*;

pub(crate) fn control_flow() {
	println!("\nBEGIN CONTROL FLOW\n");
	println!("Please input your number!");
	let number: u32;
	loop {
		let mut guess = String::new();
		match io::stdin().read_line(&mut guess) {
			Ok(string) => string,
			Err(_) => {
				println!("{}", "WARNING: Failed to read line!".bright_red());
				continue;
			}
		};
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("{}", "WARNING: Please type a number!".bright_red());
				continue;
			}
		};
		number = guess;
		break;
	}
	println!("Your number is: {}", number);
	if number < 10 {
		println!("The number is smaller than 10!");
	} else if number == 10 {
		println!("The number is equal to 10!");
	} else {
		println!("None of the conditions were met!");
	}
	let result = if number == 10 { true } else { false };
	println!("The condition that the number is 10 is: {}", result);
	println!("Now we will count until the number has been reached!");
	let mut counter = 0;
	loop {
		counter += 1;
		println!("{}", counter);
		if counter >= number {
			println!("The number has been reached!");
			break;
		}
	}
	println!("Now we will count the number down to 1!");
	let mut temp_number = number;
	while temp_number != 0 {
		println!("{}!", temp_number);
		temp_number -= 1;
	}
	println!("LIFTOFF!");
	let collection = [584858, 1222, 1244, 457678, 234, 5, 1244, 795765];
	for element in collection.iter() {
		println!("The value is: {}", element);
	}
	println!("Now we will count until the number has been reached again!");
	for num in 0..number + 1 {
		println!("{}", num);
	}
}