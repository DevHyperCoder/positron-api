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

use rocket::{
    http::Status,
    response::Responder,
    serde::json::{json, Value as JsonValue},
    Response,
};

pub struct Error {
    pub error: JsonValue,
    pub status: Status,
}

impl From<positron::error::Error> for Error {
    fn from(e: positron::error::Error) -> Self {
        Error {
            error: json!({"error":e.to_string()}),
            status: Status::BadRequest,
        }
    }
}

impl<'r> Responder<'r, 'r> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'r> {
        Response::build_from(self.error.respond_to(&request).unwrap())
            .status(self.status)
            .ok()
    }
}
