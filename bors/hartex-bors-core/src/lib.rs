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

//! # Bors Core Library

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(async_fn_in_trait)]

use std::future::Future;
use std::pin::Pin;

use octocrab::models::issues::Comment;

use crate::models::GithubRepositoryName;
use crate::models::GithubRepositoryState;
use crate::models::Permission;

pub mod models;

/// A state of bors.
pub trait BorsState<C: RepositoryClient> {
    /// Checks whether the comment is posted by bors itself.
    fn comment_posted_by_bors(&self, comment: Comment) -> bool;

    /// Returns a mutable reference to the repository state by its name.
    fn get_repository_state_mut(&mut self, repository: &GithubRepositoryName) -> Option<&mut GithubRepositoryState<C>>;
}

/// A base permission resolver.
pub trait PermissionResolver {
    /// Resolves permissions for a user and returns whether that user has the specified permission.
    fn resolve_user<'a>(
        &'a self,
        username: &'a str,
        permission: Permission,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + '_>>;
}

/// A repository client.
pub trait RepositoryClient {
    /// The name of the repository this client is for.
    fn repository_name(&self) -> &GithubRepositoryName;

    /// Post a comment on a specific pull request.
    async fn post_comment(&mut self, pr: u64, text: &str) -> hartex_eyre::Result<()>;
}
