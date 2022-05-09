use serde::{Deserialize, Serialize};

use crate::OPENAI_URL;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Model {
    Ada,
    Babbage,
    Curie,
    Davinci,
}

impl Model {
    crate fn url(&self, action: &str) -> String {
        (match self {
            Model::Ada => format!("{OPENAI_URL}/engines/text-ada-001/"),
            Model::Babbage => format!("{OPENAI_URL}/engines/text-babbage-001"),
            Model::Curie => format!("{OPENAI_URL}/engines/text-curie-001"),
            Model::Davinci => format!("{OPENAI_URL}/engines/text-davinci-002"),
        }) + action
    }
}