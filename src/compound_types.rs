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
