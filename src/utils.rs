use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

use std::collections::HashMap;

// Return an opaque 500 while preserving the error root's cause for logging.
pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

// Return a 400 with the user-representation of the validation error as body.
// The error root cause is preserved for logging purposes.
pub fn e400<T: std::fmt::Debug + std::fmt::Display>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorBadRequest(e)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

pub fn merge<T, U>(a: T, b: U) -> serde_json::Result<serde_json::Value>
where
    T: serde::Serialize,
    U: serde::Serialize,
{
    let map_a: HashMap<String, serde_json::Value> =
        serde_json::from_value(serde_json::to_value(a)?)?;
    let map_b: HashMap<String, serde_json::Value> =
        serde_json::from_value(serde_json::to_value(b)?)?;

    let mut merged_map = map_a.clone();
    for (key, value) in map_b {
        merged_map.entry(key).or_insert(value);
    }

    Ok(serde_json::to_value(merged_map)?)
}
