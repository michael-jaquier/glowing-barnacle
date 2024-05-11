use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract::Path, routing::get, Router};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Debug;
use tracing::{error, info};
use tracing::instrument;

use crate::candidate::{get_candidate, Candidate};
use crate::DemoErrors;

async fn valid_calls() -> String {
    "/hire/:person".to_string()
}

#[instrument]
pub async fn invalid_call() -> axum::http::Response<axum::body::Body> {
    error!("Invalid API call");
    DemoErrors::Error1.into_response()
}

#[instrument]
pub async fn hire_employee(Path(params): Path<CouldBeBetterParams>) -> String {
    info!("Running the hire_employee function");
    format!("Hiring {}!  ☢️ Good luck changing this API ☢️", params.person)
}

#[instrument]
pub async fn hire_employee_version(Path(params): Path<Params>) -> String {
    info!("Running the hire_employee_version function");
    if params.version == Version::V2 {
        return hire_employee_v2(Path(params)).await;
    }
    format!(
        "Hiring {} 🎉 Easy Refactors ahead 🎉 for version {:?}",
        params.person, params.version
    )
}

#[instrument]
pub async fn hire_employee_version_v3() -> (StatusCode, String) {
    info!("Running the hire_employee_version_v3 function");
    if get_candidate().name == "" {
        (StatusCode::NOT_FOUND, "No candidate found".to_string())
    } else {
        (StatusCode::OK, get_candidate().to_string())
    }
}

#[instrument]
pub async fn hire_employee_v2(Path(params): Path<Params>) -> String {
    if Candidate::is_candidate(params.person.clone()) {
        info!("Running the hire_employee_v2 function for our candidate");
        let mut tasks = String::new();
        tasks.push_str("1. Create Fantastic new Scenarios\n");
        tasks.push_str("2. Onboard new Customers\n");
        tasks.push_str("3. New Challenges!\n");
        tasks.push_str("4. Embrace learning!\n");
        tasks.push_str("5. Have Fun!\n");
        tasks
    } else {
        info!("Running the hire_employee_v2 function for a non-candidate");
        format!("Carry on with the usual tasks {}!", params.person)
    }
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum Version {
    V1,
    V2,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    version: Version,
    person: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CouldBeBetterParams {
    person: String,
}

pub fn router() -> Router {
    let v0_router = Router::new()
    .route("/hire/:person", get(hire_employee))
    .route("/help", get(valid_calls));
    let parameter_version =
        Router::new().route("/:version/hire/:person", get(hire_employee_version));

    let candidate_router = Router::new().route("/hire/", get(hire_employee_version_v3));

    let nested_router_version = Router::new()
        .nest("/v3", candidate_router)
        .nest("/", v0_router)
        .nest("/", parameter_version)
        .fallback(invalid_call);

    nested_router_version
}

/// --------------------- Ignore this code block
impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Version, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "v1" => Ok(Version::V1),
            "v2" => Ok(Version::V2),
            _ => Err(serde::de::Error::custom("invalid version")),
        }
    }
}
