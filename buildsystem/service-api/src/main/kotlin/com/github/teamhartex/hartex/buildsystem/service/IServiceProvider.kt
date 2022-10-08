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

package com.github.teamhartex.hartex.buildsystem.service

import com.github.teamhartex.hartex.buildsystem.concurrent.IStoppable
import kotlin.reflect.KClass as IKClass
import java.lang.reflect.Type as IType

interface IServiceProvider : IStoppable {
  fun getService(serviceType: IType): IService?

  fun getFactory(serviceType: IKClass<*>): IService?

  fun getAll(serviceType: IKClass<*>, visitor: IVisitor): IVisitor

  interface IVisitor {
    fun visit(service: IService)
  }
}
