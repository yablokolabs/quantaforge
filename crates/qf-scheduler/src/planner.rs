use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub circuit_json: String,
    pub shots: usize,
    pub status: JobStatus,
    pub result: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed,
}

impl Job {
    pub fn new(circuit_json: String, shots: usize) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            circuit_json,
            shots,
            status: JobStatus::Queued,
            result: None,
        }
    }
}

pub struct JobPlanner {
    jobs: Vec<Job>,
}

impl JobPlanner {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn submit(&mut self, circuit_json: String, shots: usize) -> String {
        let job = Job::new(circuit_json, shots);
        let id = job.id.clone();
        self.jobs.push(job);
        id
    }

    pub fn get_job(&self, id: &str) -> Option<&Job> {
        self.jobs.iter().find(|j| j.id == id)
    }

    pub fn get_job_mut(&mut self, id: &str) -> Option<&mut Job> {
        self.jobs.iter_mut().find(|j| j.id == id)
    }

    pub fn pending_jobs(&self) -> Vec<&Job> {
        self.jobs
            .iter()
            .filter(|j| j.status == JobStatus::Queued)
            .collect()
    }

    pub fn all_jobs(&self) -> &[Job] {
        &self.jobs
    }
}

impl Default for JobPlanner {
    fn default() -> Self {
        Self::new()
    }
}
