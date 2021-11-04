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

use std::str::FromStr;

use positron::{circuit::Circuit, parser::Parsed};
use rocket::serde::json::Json;

use crate::{
    dto::{EvaluateDto, EvaluateResult},
    error::Error,
};

#[post("/eval", data = "<body>")]
pub fn evaluate(body: Json<EvaluateDto>) -> Result<Json<EvaluateResult>, Error> {
    let parsed = Parsed::from_str(&body.0.expr)?;

    let circuit = Circuit {
        data: body.0.variable_values,
        gate: parsed.root_gate,
    };

    Ok(Json(EvaluateResult {
        result: circuit.execute()?,
    }))
}
