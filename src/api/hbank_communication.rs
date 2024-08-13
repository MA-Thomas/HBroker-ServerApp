use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

/*
For communication with HBank.

Each method here talks to HBank/.../api/endpoints.rs

E.g.,
The server app calls get_cohort_info with a cohort_id.
This sends a GET request to /api/cohort/{cohort_id} on the HBank API.
The HBank API's get_cohort_info function handles this request.
It uses the DataManager to retrieve the cohort info.
The API returns the CohortInfo as a JSON response.
The server app deserializes this response into a CohortInfo struct.

*/


#[derive(Debug, Serialize, Deserialize)]
pub struct CohortInfo {
    pub cohort_id: String,
    pub size: usize,
    pub data_types: Vec<String>,
    // Add other relevant fields
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyntheticDataSetup {
    pub cohort_id: String,
    pub data_dir: PathBuf,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeSubmission {
    pub cohort_id: String,
    pub code: String,
    pub dependencies: Vec<String>,
    pub data_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub job_id: String,
    pub status: String,
    pub result: Option<String>,
    pub error: Option<String>,
}

pub struct HBankCommunication {
    client: Client,
    server_app_id: String,
    hbank_base_url: String,
}

impl HBankCommunication {
    pub fn new(server_app_id: String, hbank_base_url: String) -> Self {
        HBankCommunication {
            client: Client::new(),
            server_app_id,
            hbank_base_url,
        }
    }

    pub async fn get_cohort_info(&self, cohort_id: &str) -> Result<CohortInfo, reqwest::Error> {
        let url = format!("{}/api/cohort/{}", self.hbank_base_url, cohort_id);
        self.client.get(&url)
            .header("Server-App-ID", &self.server_app_id)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn setup_synthetic_data_dir(&self, cohort_id: &str, server_app_data_dir: &str) -> Result<SyntheticDataSetup, reqwest::Error> {
        let url = format!("{}/api/synthetic/setup", self.hbank_base_url);
        self.client.post(&url)
            .header("Server-App-ID", &self.server_app_id)
            .json(&serde_json::json!({
                "cohort_id": cohort_id,
                "server_app_data_dir": server_app_data_dir
            }))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn request_synthetic_cohort_data(&self, cohort_id: &str, server_app_data_dir: &str) -> Result<(), reqwest::Error> {
        let url = format!("{}/api/synthetic/data", self.hbank_base_url);
        self.client.post(&url)
            .header("Server-App-ID", &self.server_app_id)
            .json(&serde_json::json!({
                "cohort_id": cohort_id,
                "server_app_data_dir": server_app_data_dir
            }))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn run_on_synthetic_data(&self, submission: &CodeSubmission) -> Result<AnalysisResult, reqwest::Error> {
        let url = format!("{}/api/run/synthetic", self.hbank_base_url);
        self.client.post(&url)
            .header("Server-App-ID", &self.server_app_id)
            .json(submission)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn run_on_sensitive_data(&self, submission: &CodeSubmission) -> Result<AnalysisResult, reqwest::Error> {
        let url = format!("{}/api/run/sensitive", self.hbank_base_url);
        self.client.post(&url)
            .header("Server-App-ID", &self.server_app_id)
            .json(submission)
            .send()
            .await?
            .json()
            .await
    }
}