/*
 * positron-api - REST API Wrapper around positron
 * Copyright (C) 2021 DevHyperCoder
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#[macro_use]
pub extern crate rocket;

pub mod cors;
pub mod dto;
pub mod error;
pub mod routes;

use rocket::{Build, Rocket};

use crate::cors::CORS;
use crate::routes::evaluate::evaluate;
use crate::routes::index::index;
use crate::routes::truth_table::truth_table;

pub fn get_rocket() -> Rocket<Build> {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, evaluate, truth_table])
}
