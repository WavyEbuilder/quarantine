/*
 * Copyright (C) 2024 WavyEbuilder
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://www.gnu.org/licenses/.
 *
 * SPDX-License-Identifier: GPL-3.0-only
 */

use std::{
    process::ExitCode,
    env,
};
use crate::load::load_profile;

mod load;
mod profile;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <profile_name>", args[0]);
        return ExitCode::FAILURE;
    }

    let profile_name = &args[1];

    match load_profile(profile_name) {
        Ok(profile) => {
            println!("{:#?}", profile);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
