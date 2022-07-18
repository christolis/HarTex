/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The HarTex REST Process Binary
//!
//! The REST process binary acts as a proxy over the Discord API.

#![deny(clippy::pedantic)]
#![deny(warnings)]
#![feature(async_closure)]
#![feature(let_else)]
#![allow(clippy::let_underscore_drop)]
#![allow(clippy::match_result_ok)]

use std::env as stdenv;

use base::cmdline;
use base::error::Result;
use base::logging;
use base::panicking;
use hyper::{Body, Client};
use hyper_rustls::HttpsConnectorBuilder;
use hyper_trust_dns::{TrustDnsHttpConnector, TrustDnsResolver};
use rest::RestState;

mod proxy;

#[tokio::main]
pub async fn main() -> Result<()> {
    logging::init();
    panicking::init();

    let args = stdenv::args().collect::<Vec<_>>();
    let args = &args[1..];
    let mut base_options = cmdline::Options::new();
    let options = base_options.reqopt(
        "",
        "port",
        "The port for the event server to run on",
        "PORT",
    );
    let Some(matches) = options.parse(args).ok() else {
        log::error!("could not parse command line arguments; exiting");

        return Ok(());
    };
    let Ok(Some(port)) = matches.opt_get::<u16>("port") else {
        log::error!("could not parse port argument; exiting");

        return Ok(());
    };

    if let Err(error) = env::load() {
        log::error!("env error: {error}");
        log::warn!("environment variables cannot be loaded; exiting");
        log::info!("help: please make sure that the required environment variables are present");
        log::info!(r#"help: see the above errors (those that start with "retrieval failed")"#);

        return Ok(());
    }

    log::trace!("creating https connector for http client");
    let https_connector = {
        let mut connector = TrustDnsHttpConnector::new_with_resolver(TrustDnsResolver::new());
        connector.enforce_http(false);

        let builder = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_only()
            .enable_http1();

        if let Ok(enabled) = stdenv::var("REST_HTTP_2_ENABLED") && matches!(enabled.as_str(), "true") {
            builder.enable_http2().wrap_connector(connector)
        } else {
            builder.wrap_connector(connector)
        }
    };

    log::trace!("creating http client");
    let client = Client::builder().build::<_, Body>(https_connector);

    log::trace!("creating http server");
    let mut server = tide::with_state(RestState::new(client));
    server.at("/*").post(proxy::proxy_request);

    log::trace!("listening on port {port}");
    if let Err(error) = server.listen(format!("127.0.0.1:{port}")).await {
        log::error!("server error: could not listen on port {port}: {error}");
    }

    Ok(())
}