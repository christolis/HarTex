/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

//! # The website for bors.
//!
//! The website is home to the "bors cheatsheet" as well as the queues for certain repositories.

#![feature(lazy_cell)]

use std::sync::LazyLock;

use handlebars::Handlebars;
use hartex_log::log;

mod index;

pub(crate) static HANDLEBARS: LazyLock<Handlebars> = LazyLock::new(|| {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_templates_directory(".hbs", "./templates").unwrap();

    handlebars
});

/// The entry point.
#[rocket::main]
pub async fn main() -> hartex_eyre::Result<()> {
    hartex_eyre::initialize()?;
    hartex_log::initialize();

    log::trace!("initializing handlebars instance");
    LazyLock::force(&HANDLEBARS);

    log::debug!("igniting rocket");
    let rocket = rocket::build()
        .ignite()
        .await?;

    log::debug!("launching rocket");
    rocket.launch().await?;

    Ok(())
}
