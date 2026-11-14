pub struct AzureOpenAIConfig {
    endpoint: String,
    deployment: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AzureOpenAIConfigError {
    EmptyDeployment,
    EmptyEndpoint,
}

impl AzureOpenAIConfig {
    pub fn new(
        endpoint: String,
        deployment: String,
    ) -> Result<Self, AzureOpenAIConfigError> {
        if deployment.is_empty() {
            return Err(AzureOpenAIConfigError::EmptyDeployment);
        }
        if endpoint.is_empty(){
            return Err(AzureOpenAIConfigError::EmptyEndpoint);
        }

        Ok(Self {
            endpoint,
            deployment,
        })
        
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn deployment(&self) -> &str {
        &self.deployment
    }
}