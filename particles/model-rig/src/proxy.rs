use async_trait::async_trait;
use rig::completion::{
    CompletionError, CompletionModel, CompletionRequest,
    CompletionResponse as RigCompletionResponse,
};
use rig::providers::openai;

pub type CompletionResponse = RigCompletionResponse<()>;

#[async_trait]
pub trait VCompletionModel: Send {
    async fn completion(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse, CompletionError>;
}

#[async_trait]
impl VCompletionModel for openai::CompletionModel {
    async fn completion(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse, CompletionError> {
        let response = CompletionModel::completion(self, request).await?;
        let clipped_response = CompletionResponse {
            choice: response.choice,
            raw_response: (),
        };
        Ok(clipped_response)
    }
}
