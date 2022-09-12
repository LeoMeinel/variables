/*
 * File: mutable_shadowing_const.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) fn mutable_shadowing_const() {
    println!("\nBEGIN MUTABLE SHADOWING CONST\n");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let y = 0;
    println!("The value of y is: {}", y);
    let y = 1;
    println!("The value of y is: {}", y);
    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("The value of SUBSCRIBER_COUNT is: {}", SUBSCRIBER_COUNT);
}
