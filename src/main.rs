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
    fs,
    path::PathBuf,
    os::unix::fs::PermissionsExt,
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

    let xdg_runtime_dir = match env::var("XDG_RUNTIME_DIR") {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Error: XDG_RUNTIME_DIR is not set");
            return ExitCode::FAILURE;
        }
    };

    {
        let app_dir = PathBuf::from(xdg_runtime_dir).join("quarantine");

        if let Err(e) = fs::create_dir_all(&app_dir) {
            eprintln!("Error: Could not create runtime directory: {}", e);
            return ExitCode::FAILURE;
        }

        let metadata = match fs::metadata(&app_dir) {
            Ok(meta) => meta,
            Err(e) => {
                eprintln!("Error: Could not fetch runtime directory metadata: {}", e);
                return ExitCode::FAILURE;
            }
        };

        let mut perms = metadata.permissions();
        perms.set_mode(0o700);

        if let Err(e) = fs::set_permissions(&app_dir, perms) {
            eprintln!("Error: Could not set runtime directory permissions: {}", e);
            return ExitCode::FAILURE;
        }
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
