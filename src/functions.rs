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

pub(crate) fn functions() {
    simple_function(11, -22);
    let z = return_function(11, -22);
    println!("\nBEGIN MAIN FUNCTION\n");
    println!("The value of z is: {}", z);
}

fn simple_function(x: u32, y: i32) {
    println!("\nBEGIN SIMPLE FUNCTION\n");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn return_function(x: i32, y: i32) -> i32 {
    println!("\nBEGIN RETURN FUNCTION\n");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}
