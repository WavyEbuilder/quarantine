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

#![allow(dead_code)]

use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid data type for field '{0}': expected {1}")]
    InvalidDataType(String, String),
    #[error("TOML parsing error: {0}")]
    TomlParseError(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub app: App,
    pub binary: Binary,
    pub share: Share,
    pub filesystem: Filesystem,
    pub socket: Socket,
    pub devices: Devices,
}

#[derive(Debug, Deserialize)]
pub struct App {
    pub name: String,
    pub identifier: String,
}

#[derive(Debug, Deserialize)]
pub struct Binary {
    pub path: String,
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Share {
    #[serde(default)]
    pub net: bool,
    #[serde(default)]
    pub ipc: bool,
}

#[derive(Debug, Deserialize)]
pub struct Filesystem {
    #[serde(default)]
    pub ro_bind: Vec<String>,
    #[serde(default)]
    pub rw_bind: Vec<(String, String)>,
    #[serde(default)]
    pub symlinks: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
pub struct Socket {
    #[serde(default)]
    pub pipewire: bool,
    #[serde(default)]
    pub pulseaudio: bool,
    #[serde(default)]
    pub session_bus: bool,
    #[serde(default)]
    pub system_bus: bool,
    #[serde(default)]
    pub wayland: bool,
}

#[derive(Debug, Deserialize)]
pub struct Devices {
    #[serde(default)]
    pub all: bool,
    #[serde(default)]
    pub dri: bool,
    #[serde(default)]
    pub kvm: bool,
    #[serde(default)]
    pub shm: bool,
}

macro_rules! validate_non_empty {
    ($profile:expr, $field:expr, $field_name:expr) => {
        if $field.is_empty() {
            return Err(ProfileError::MissingField($field_name.to_string()));
        }
    };
}

impl Profile {
    pub fn from_toml(toml_str: &str) -> Result<Self, ProfileError> {
        let profile: Profile = toml::from_str(toml_str)?;

        validate_non_empty!(profile, profile.app.name, "app.name");
        validate_non_empty!(profile, profile.app.identifier, "app.identifier");
        validate_non_empty!(profile, profile.binary.path, "binary.path");

        Ok(profile)
    }
}
