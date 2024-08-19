use h_bank::api_prelude::*;

pub struct HBankCommunication {
    hbank: HBankInterface,
}

impl HBankCommunication {
    pub fn new() -> Self {
        Self {
            hbank: HBankInterface::new(),
        }
    }

    pub fn get_cohort_info(&self, cohort_id: &str) -> Result<CohortSummary, String> {
        self.hbank.get_cohort_summary(cohort_id)
    }

    pub fn submit_code(&self, submission: CodeSubmission) -> Result<String, String> {
        self.hbank.submit_code(submission)
    }

    pub fn setup_synthetic_data(&self, setup: SyntheticDataSetup) -> Result<SyntheticDataSetup, String> {
        self.hbank.setup_synthetic_data(setup)
    }

    pub fn submit_analysis_result(&self, result: AnalysisResult) -> Result<(), String> {
        self.hbank.submit_analysis_result(result)
    }

    // Add other methods as needed...
}