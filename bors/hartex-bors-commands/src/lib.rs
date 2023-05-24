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

//! # Commands for Bors

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

pub mod commands;
pub mod parser;

/// Represents a command.
#[derive(Debug)]
pub enum BorsCommand {
    /// Ping command.
    ///
    /// `bors ping`
    Ping,
}

/// Parses bors commands from an input string.
pub fn parse_commands(input: &str) -> Vec<Result<BorsCommand, parser::ParserError>> {
    let parsers: Vec<fn(parser::Parser) -> parser::ParserResult> = vec![parser::parse_ping];

    input
        .lines()
        .filter_map(|line| match line.find(parser::PREFIX) {
            Some(position) => {
                let command = &line[position + parser::PREFIX.len()..];

                for callback in &parsers {
                    if let Some(result) = callback(parser::Parser::new(command)) {
                        return Some(result);
                    }
                }

                parser::parse_remaining(parser::Parser::new(command))
            }
            None => None,
        })
        .collect()
}