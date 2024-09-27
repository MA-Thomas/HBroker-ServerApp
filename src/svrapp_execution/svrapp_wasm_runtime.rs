use async_trait::async_trait;
use wasmtime::*;
use crate::svrapp_execution::svrapp_vm_trait::{VirtualMachine, JobStatus};
use crate::svrapp_api::shared_models::{CodeSubmission, AnalysisResult};
use std::error::Error;

pub struct WasmRuntime {
    job_id: Option<String>,
    status: JobStatus,
    engine: Engine,
    module: Option<Module>,
    instance: Option<Instance>,
}

impl WasmRuntime {
    pub fn new() -> Self {
        WasmRuntime {
            job_id: None,
            status: JobStatus::Queued,
            engine: Engine::default(),
            module: None,
            instance: None,
        }
    }
}

#[async_trait]
impl VirtualMachine for WasmRuntime {
    async fn create(&mut self, job_id: &str, submission: &CodeSubmission) -> Result<(), Box<dyn Error>> {
        self.job_id = Some(job_id.to_string());
        self.status = JobStatus::Queued;
        
        // Compile the WASM module
        self.module = Some(Module::new(&self.engine, &submission.code)?);
        
        Ok(())
    }

    async fn execute_job(&mut self) -> Result<AnalysisResult, Box<dyn Error>> {
        self.status = JobStatus::Running;
        
        let mut store = Store::new(&self.engine, ());
        let instance = Instance::new(&mut store, self.module.as_ref().unwrap(), &[])?;
        
        // Assuming the WASM module exports a "run" function
        let run = instance.get_typed_func::<(), i32>(&mut store, "run")?;
        let result = run.call(&mut store, ())?;
        
        self.status = JobStatus::Completed;
        
        Ok(AnalysisResult {
            job_id: self.job_id.clone().unwrap(),
            status: "completed".to_string(),
            result: Some(result.to_string()),
            error: None,
        })
    }

    async fn get_status(&self) -> JobStatus {
        self.status.clone()
    }

    async fn cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        self.module = None;
        self.instance = None;
        Ok(())
    }
}