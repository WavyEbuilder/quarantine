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

use std::path::Path;
use std::fs;
use thiserror::Error;

use crate::profile::{Profile, ProfileError};

// As we iterate through the directories to search in order, let's ensure
// we list them from first to be searched to last to be searched, so that
// directories with a higher order of precedence are searched first.
const PROFILE_SEARCH_DIRS: &[&str] = &["/etc/quarantine", "/usr/share/quarantine"];

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("Profile not found")]
    NotFound,
    #[error("Invalid profile: {0}")]
    InvalidProfile(#[from] ProfileError),
    #[error("Invalid TOML: {0}")]
    InvalidToml(#[from] toml::de::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn load_profile(profile_name: &str) -> Result<Profile, LoadError> {
    let profile_filename = format!("{}.profile", profile_name);

    for dir in PROFILE_SEARCH_DIRS {
        let search_dir = Path::new(dir);
        if let Ok(entries) = fs::read_dir(search_dir) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)
                   && entry.file_name().to_str() == Some(&profile_filename) {
                    let path = entry.path();
                    let content = fs::read_to_string(&path)?;
                    return Profile::from_toml(&content).map_err(LoadError::InvalidProfile);
                }
            }
        }
    }
    Err(LoadError::NotFound)
}
