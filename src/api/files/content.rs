use crate::api::BuildRequest;
use crate::OPENAI_URL;
use serde::{Deserialize, Serialize};

/// # OpenAi documentation
///
/// Returns the contents of the specified file
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub file_id: String,
}
impl Request {
    pub fn new(file_id: String) -> Self {
        Request { file_id }
    }
}
impl BuildRequest for Request {
    fn build_request(&self, client: &crate::Client) -> crate::RequestBuilder {
        client
            .reqwest_client()
            .get(format!("{OPENAI_URL}/files/{}/content", self.file_id))
            .bearer_auth(client.gpt_token())
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response {
    pub content: String,
}
#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
impl crate::client::Request for Request {
    type Response = Response;

    #[cfg(not(feature = "blocking"))]
    async fn request(
        &self,
        client: &crate::Client,
    ) -> reqwest::Result<<Self as crate::client::Request>::Response> {
        Ok(Response {
            content: self.request_raw(client).await?,
        })
    }
    #[cfg(feature = "blocking")]
    fn request(
        &self,
        client: &crate::Client,
    ) -> reqwest::Result<<Self as crate::client::Request>::Response> {
        Ok(Response {
            content: self.request_raw(client)?,
        })
    }
}
