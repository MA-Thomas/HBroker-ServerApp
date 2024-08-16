use async_trait::async_trait;
use crate::api::shared_models::{CodeSubmission, AnalysisResult};
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed,
}

#[async_trait]
pub trait VirtualMachine: Send + Sync {
    async fn create(&mut self, job_id: &str, submission: &CodeSubmission) -> Result<(), Box<dyn Error>>;
    async fn transfer_code_and_data(&mut self, submission: &CodeSubmission) -> Result<(), Box<dyn Error>>;
    async fn execute_job(&mut self) -> Result<AnalysisResult, Box<dyn Error>>;
    async fn get_status(&self) -> JobStatus;
    async fn cleanup(&mut self) -> Result<(), Box<dyn Error>>;
}