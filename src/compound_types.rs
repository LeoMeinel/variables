/*
 * File: compound_types.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) fn compound_types() {
    println!("\nBEGIN TUPLE\n");
    let tup = ("Let's Get Rusty!", 100_000);
    let (channel, sub_count) = tup;
    println!(
        "tup consists of channel: {} and sub_count: {}",
        channel, sub_count
    );
    let vehicle = ("Porsche", 220);
    let model = vehicle.0;
    let velocity = vehicle.1;
    println!(
        "vehicle consists of model: {} and velocity: {}",
        model, velocity
    );
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    println!(
        "The values of error_codes are: {}, {} and {}",
        error_codes[0], error_codes[1], error_codes[2]
    );
    println!("The value of not_found is: {}", not_found);
    let byte = [0; 8];
    println!(
        "The values of byte are: {}, {}, {}, {}, {}, {}, {} and {}",
        byte[0], byte[1], byte[2], byte[3], byte[4], byte[5], byte[6], byte[7]
    );
}
