use reqwest::Client;
use crate::api::shared_models::{CodeSubmission, AnalysisResult, SyntheticDataSetup, CohortInfo};
use std::error::Error;

pub struct HBankCommunication {
    client: Client,
    base_url: String,
    server_app_id: String,
}

impl HBankCommunication {
    pub fn new(base_url: String, server_app_id: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            server_app_id,
        }
    }

    pub async fn get_cohort_info(&self, cohort_id: &str) -> Result<CohortInfo, Box<dyn Error>> {
        let url = format!("{}/api/cohort/{}", self.base_url, cohort_id);
        let response = self.client.get(&url)
            .header("Server-App-ID", &self.server_app_id)
            .send()
            .await?;
        
        let cohort_info = response.json::<CohortInfo>().await?;
        Ok(cohort_info)
    }

    pub async fn request_synthetic_cohort_data(&self, setup: &SyntheticDataSetup) -> Result<SyntheticDataSetup, Box<dyn Error>> {
        let url = format!("{}/api/synthetic/data", self.base_url);
        let response = self.client.post(&url)
            .header("Server-App-ID", &self.server_app_id)
            .json(setup)
            .send()
            .await?;
        
        let result = response.json::<SyntheticDataSetup>().await?;
        Ok(result)
    }

    pub async fn run_on_synthetic_data(&self, submission: &CodeSubmission) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/run/synthetic", self.base_url);
        let response = self.client.post(&url)
            .header("Server-App-ID", &self.server_app_id)
            .json(submission)
            .send()
            .await?;
        
        let result = response.json::<serde_json::Value>().await?;
        Ok(result["job_id"].as_str().unwrap().to_string())
    }

    pub async fn run_on_sensitive_data(&self, submission: &CodeSubmission) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/run/sensitive", self.base_url);
        let response = self.client.post(&url)
            .header("Server-App-ID", &self.server_app_id)
            .json(submission)
            .send()
            .await?;
        
        let result = response.json::<serde_json::Value>().await?;
        Ok(result["job_id"].as_str().unwrap().to_string())
    }

    pub async fn setup_synthetic_data_dir(&self, setup: &SyntheticDataSetup) -> Result<SyntheticDataSetup, Box<dyn Error>> {
        let url = format!("{}/api/synthetic/setup", self.base_url);
        let response = self.client.post(&url)
            .header("Server-App-ID", &self.server_app_id)
            .json(setup)
            .send()
            .await?;
        
        let result = response.json::<SyntheticDataSetup>().await?;
        Ok(result)
    }
}