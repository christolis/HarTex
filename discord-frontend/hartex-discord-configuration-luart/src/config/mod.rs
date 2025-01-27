/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # Configuration Object
//!
//! This is the lowest level in the configuration structure hierarchy.

use mlua::Error;
use mlua::FromLuaMulti;
use mlua::Lua;
use mlua::MultiValue;
use mlua::Value;

use crate::config::appearance::Appearance;
use crate::config::dashboard::Dashboard;
use crate::config::plugins::Plugins;

pub mod appearance;
pub mod dashboard;
pub mod plugins;

/// The configuration object.
#[derive(Debug)]
pub struct Configuration {
    /// An optional appearance configuration object.
    pub appearance: Option<Appearance>,
    /// A dashboard configuration object.
    pub dashboard: Dashboard,
    /// An optional plugins configuration object.
    pub plugins: Option<Plugins>,
}

impl<'lua> FromLuaMulti<'lua> for Configuration {
    fn from_lua_multi(values: MultiValue<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        if values.is_empty() {
            return Err(Error::RuntimeError(String::from(
                "Configuration: multi value is empty",
            )));
        }

        let next_value = values.into_iter().next().unwrap();
        let Value::Table(value) = next_value.clone() else {
            return Err(Error::RuntimeError(format!(
                "Configuration: mismatched value type, exoected table, found: {}",
                next_value.type_name()
            )));
        };

        let appearance = value.get("appearance")?;
        let dashboard = value.get("dashboard")?;
        let plugins = value.get("plugins")?;

        Ok(Self {
            appearance,
            dashboard,
            plugins,
        })
    }
}
