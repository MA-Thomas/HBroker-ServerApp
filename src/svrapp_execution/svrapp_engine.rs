use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::api::shared_models::{CodeSubmission, AnalysisResult};
use super::vm_trait::{VirtualMachine, JobStatus};
use super::vm_factory::VMFactory;

pub struct ExecutionEngine {
    job_queue: Vec<(String, CodeSubmission)>,
    active_vms: HashMap<String, Mutex<Box<dyn VirtualMachine>>>,
    vm_factory: VMFactory,
}

#[async_trait]
pub trait ExecutionEngineTrait: Send + Sync {
    async fn queue_job(&self, submission: &CodeSubmission) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_job_status(&self, job_id: &str) -> Option<String>;
    async fn run_next_job(&mut self) -> Option<Result<AnalysisResult, Box<dyn std::error::Error + Send + Sync>>>;
}

impl ExecutionEngine {
    pub fn new(vm_factory: VMFactory) -> Self {
        ExecutionEngine {
            job_queue: Vec::new(),
            active_vms: HashMap::new(),
            vm_factory,
        }
    }
}

#[async_trait]
impl ExecutionEngineTrait for ExecutionEngine {
    // async fn queue_job(&mut self, job_id: &str, submission: &CodeSubmission) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //     self.job_queue.push((job_id.to_string(), submission.clone()));
    //     Ok(())
    // }
    async fn queue_job(&self, submission: &CodeSubmission) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.job_queue.push(submission.clone());
        Ok(())
    }

    async fn get_job_status(&self, job_id: &str) -> Option<JobStatus> {
        if let Some(vm) = self.active_vms.get(job_id) {
            let vm = vm.lock().await;
            Some(vm.get_status().await)
        } else {
            None
        }
    }

    async fn run_next_job(&mut self) -> Option<Result<AnalysisResult, Box<dyn std::error::Error + Send + Sync>>> {
        let (job_id, submission) = self.job_queue.pop()?;

        let result = async {
            let mut vm = self.vm_factory.create_vm().await;
            vm.create(&job_id, &submission).await?;
            vm.transfer_code_and_data(&submission).await?;
            let result = vm.execute_job().await?;
            self.active_vms.insert(job_id.clone(), Mutex::new(vm));
            Ok(result)
        }.await;

        Some(result)
    }
}