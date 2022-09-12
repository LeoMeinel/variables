/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
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
