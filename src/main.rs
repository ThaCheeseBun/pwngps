/*
pwngps: paw-gps but less hacky for pwnagotchi.
Copyright (C) 2023  ThaCheeseBun

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::convert::Infallible;
use tokio::process::Command;
use warp::http::{Response, StatusCode};
use warp::Filter;
use chrono::{Utc, Duration};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Location {
    latitude: f64,
    longitude: f64,
    altitude: f64,
    accuracy: f64,
    vertical_accuracy: f64,
    bearing: f64,
    speed: f64,
    elapsedMs: i64,
    provider: String,
}

async fn gps(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    let provider = match query.get("p") {
        Some(v) => v,
        None => "gps",
    };

    let request = match query.get("r") {
        Some(v) => v,
        None => "once",
    };

    let output = Command::new("termux-location")
        .arg(format!("-p{}", provider))
        .arg(format!("-r{}", request))
        .output()
        .await
        .unwrap();
    let exit_code = output.status.code().unwrap();

    if exit_code == 0 && output.stdout.len() > 0 {
        let parsed: Location = serde_json::from_slice(&output.stdout).unwrap();

        let utc = Utc::now()
            .checked_sub_signed(Duration::milliseconds(parsed.elapsedMs))
            .unwrap()
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

        return Ok(Response::builder()
            .status(StatusCode::OK)
            .body(json!({
                "Latitude": parsed.latitude,
                "Longitude": parsed.longitude,
                "Altitude": parsed.altitude,
                "Updated": utc
            }).to_string()));
    } else {
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(json!({
                "error": "Something went wrong",
                "code": exit_code
            }).to_string()));
    }
}

#[tokio::main]
async fn main() {
    let api = warp::path!("api" / "v1" / "gps")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(gps);

    let legacy = warp::path!("gps.xhtml")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(gps);

    println!("yooo we gaming on 0.0.0.0:42069");

    warp::serve(api.or(legacy)).run(([0, 0, 0, 0], 42069)).await;
}
