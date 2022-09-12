/*
 * File: functions.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
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
