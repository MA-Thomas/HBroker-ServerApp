use serde::{Serialize, Deserialize};
use std::path::PathBuf;

/*
The pub use statement is doing double duty here:
It's importing the types from h_bank::api_prelude.
It's also making these types available (re-exporting them) to any module that imports from svrapp_shared_models.
*/
pub use h_bank::api_prelude::{
    SyntheticDataSetup,
    CodeSubmission,
    AnalysisResult,
    DataPrivacyLevel,
    CohortSummary,
};



// Define any additional types specific to HBroker-ServerApp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerAppConfig {
    pub hbank_url: String,
    pub server_app_id: String,
    pub max_concurrent_jobs: usize,
}

// You can create type aliases if needed
pub type HBankAnalysisResult = AnalysisResult;

// If you need to modify a type from HBank, you can wrap it:
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedCodeSubmission {
    #[serde(flatten)]
    pub base: CodeSubmission,
    pub additional_metadata: String,
}

// Add any other HBroker-ServerApp specific structures here