use std::fmt::Display;

use serde::{Deserialize, Serialize};

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
        if self.name == "" {
            write!(f, "No candidate found")
        } else {
            write!(
                f,
                "Welcome! {} to the interview process for {} at Nexthink!",
                self.name, self.position
            )
        }
    }
}
