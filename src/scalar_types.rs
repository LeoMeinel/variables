/*
 * File: scalar_types.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) fn scalar_types() {
    // Integers
    println!("\nBEGIN INTEGERS\n");
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);
    println!("The value of d is: {}", d);
    println!("The value of e is: {}", e);
    // Floating-point numbers
    println!("\nBEGIN FLOATING-POINT NUMBERS\n");
    let f = 2.0;
    let g = 3.0;
    println!("The value of f is: {}", f);
    println!("The value of g is: {}", g);
    let sum = f + g;
    let difference = f - g;
    let product = f * g;
    let quotient = f / g;
    let remainder = f % g;
    println!("The sum of f and g is: {}", sum);
    println!("The difference of f and g is: {}", difference);
    println!("The product of f and g is: {}", product);
    println!("The quotient of f and g is: {}", quotient);
    println!("The remainder of f and g is: {}", remainder);
    // Booleans
    println!("\nBEGIN BOOLEANS\n");
    let h = true;
    let j = false;
    if h {
        println!("The value of h is: {}", h);
    }
    if !j {
        println!("The value of j is: {}", j);
    }
    // Character
    println!("\nBEGIN CHARACTERS\n");
    let k = '☀';
    println!("The value of k is: {}", k);
}
