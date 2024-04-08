use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate, PartialEq, Eq)]
pub struct UserJobs {
    #[validate(length(min = 1, message = "must not be empty"))]
    jobs: String,
}

impl UserJobs {
    pub fn new(jobs: &str) -> Self {
        Self {
            jobs: jobs.trim().to_string(),
        }
    }
}

impl ToString for UserJobs {
    fn to_string(&self) -> String {
        self.jobs.to_string()
    }
}
