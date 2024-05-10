use axum::http::StatusCode;
use axum::{extract::Path, routing::get, Router};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Debug;
use std::fmt::Display;
use tracing::info;
use tracing::instrument;

pub async fn invalid_call() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "Invalid call".to_string())
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
    V3,
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
    Router::new()
        .route("/:version/hire/:person", get(hire_employee_version))
        .route("/hire/:person", get(hire_employee))
        .fallback(get(invalid_call))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Candidate {
    pub name: String,
    pub position: String,
}

impl Candidate {
    pub fn is_candidate(person: String) -> bool {
        person.to_lowercase() == get_candidate().name.to_lowercase()
    }
}

pub fn get_candidate() -> Candidate {
    let yaml_data = include_str!("../candidate.yml");
    let candidate: Candidate = serde_yaml::from_str(yaml_data).unwrap();
    candidate
}

impl Display for Candidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Welcome! {} to the interview process for Position: {}",
            self.name, self.position
        )
    }
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
            "v3" => Ok(Version::V3),
            _ => Err(serde::de::Error::custom("invalid version")),
        }
    }
}
