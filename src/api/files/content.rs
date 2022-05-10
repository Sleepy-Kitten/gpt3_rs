use serde::{Deserialize, Serialize};
use crate::api::{Action, Auth};
use crate::OPENAI_URL;

struct Request {
    file_id: String,
}

impl Action for Request {
    type Response = Response;

    fn build_request(&self, client: &crate::Client) -> reqwest::RequestBuilder {
        client
            .reqwest_client()
            .delete(format!("{OPENAI_URL}/files/{}/content", self.file_id))
            .auth(client.gpt_token())
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub content: String,
}