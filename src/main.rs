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
 * along with this program. If not, see https://github.com/LeoMeinel/project/blob/main/LICENSE
 */

use crate::compound_types::compound_types;
use crate::control_flow::control_flow;
use crate::functions::functions;
use crate::mutable_shadowing_const::mutable_shadowing_const;
use crate::scalar_types::scalar_types;

mod compound_types;
mod control_flow;
mod functions;
mod mutable_shadowing_const;
mod scalar_types;

fn main() {
    mutable_shadowing_const();
    scalar_types();
    compound_types();
    functions();
    control_flow();
}
