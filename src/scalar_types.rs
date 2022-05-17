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
