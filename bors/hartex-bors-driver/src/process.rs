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

//! # The Bors Process

use std::future::Future;

use futures_util::StreamExt;
use hartex_bors_github::GithubBorsState;
use hartex_log::log;
use reqwest_eventsource::Event;
use reqwest_eventsource::EventSource;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use crate::event::BorsEvent;

/// Create a bors process.
pub fn bors_process(_: GithubBorsState) -> impl Future<Output = ()> {
    let service = async move {
        let mut event_source = EventSource::new("https://smee.io/0hxbLZ8FapSmKi1E")?;
        while let Some(event) = event_source.next().await {
            match event {
                Ok(Event::Open) => log::trace!("eventsource connection opened"),
                Ok(Event::Message(message)) => {
                    log::trace!("received event of type {}", message.event);
                    // todo: deserialize and handle
                },
                Err(error) => log::error!("an error occurred"),
            }
        }
    };

    service
}
