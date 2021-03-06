//! Classifies a query from provided context
//! # Builder
//! Use the [`classifications::Builder`][struct@Builder] to construct a [`classifications::Request`][Request] struct
use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::into_vec::IntoVec;
use crate::model::Model;
use crate::OPENAI_URL;

use super::{LogProbs, RequestInfo};
/// Classifies a query from provided context
///
/// # OpenAi documentation
/// Classifies the specified query using provided examples.
///
/// The endpoint first searches over the labeled examples to select the ones most relevant for the particular query.
/// Then, the relevant examples are combined with the query to construct a prompt to produce the final label via the completions endpoint.
/// Labeled examples can be provided via an uploaded file, or explicitly listed in the request using the examples parameter for quick tests and small scale use cases.
///
/// # Example
/// ```ignore
/// let request = classifications::Builder::default()
///     .model(Model::Curie)
///     .search_model(Model::Ada)
///     .query("It is a rainy day :(")
///     .examples(&[
///         &["A happy moment", "Positive"],
///         &["I am sad.", "Negative"],
///         &["I am feeling awesome", "Positive"]
///      ])
///     .labels(&["Positive", "Negative", "Neutral"])
///     .build()
///     .unwrap();
/// ```
/// # Required
/// ```ignore
/// model, query
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[builder_struct_attr(doc = "# Required")]
#[builder_struct_attr(doc = "[`model`](Self::model())")]
#[builder_struct_attr(doc = "[`query`](Self::query())")]
#[builder_struct_attr(doc = "")]
#[builder(name = "Builder")]
pub struct Request {
    /// ID of the engine to use for completion. You can select one of ada, babbage, curie, or davinci.
    pub model: Model,
    /// Query to be classified.
    #[builder(setter(into))]
    pub query: String,
    /// A list of examples with labels, in the following format:
    /// `[["The movie is so interesting.", "Positive"], ["It is quite boring.", "Negative"], ...]`
    /// All the label strings will be normalized to be capitalized.
    /// You should specify either examples or file, but not both.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<IntoVec<Vec<String>>>,
    /// The ID of the uploaded file that contains training examples. See upload file for how to upload a file of the desired format and purpose.
    /// You should specify either examples or file, but not both.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// The set of categories being classified. If not specified, candidate labels will be automatically collected from the examples you provide.
    /// All the label strings will be normalized to be capitalized.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<IntoVec<String>>,
    /// ID of the engine to use for Search. You can select one of ada, babbage, curie, or davinci.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_model: Option<Model>,
    /// What sampling temperature to use. Higher values mean the model will take more risks.
    /// Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Include the log probabilities on the logprobs most likely tokens, as well the chosen tokens. For example, if logprobs is 5, the API will return a list of the 5 most likely tokens.
    ///  The API will always return the logprob of the sampled token, so there may be up to logprobs+1 elements in the response.
    /// The maximum value for logprobs is 5. If you need more than this, please contact support@openai.com and describe your use case.
    /// When logprobs is set, completion will be automatically added into expand to get the logprobs.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u8>,
    /// The maximum number of examples to be ranked by Search when using file.
    /// Setting it to a higher value leads to improved accuracy but with increased latency and cost.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_examples: Option<u64>,
    /// Modify the likelihood of specified tokens appearing in the completion.
    /// Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100.
    /// You can use this tokenizer tool (which works for both GPT-2 and GPT-3) to convert text to token IDs.
    /// Mathematically, the bias is added to the logits generated by the model prior to sampling.
    /// The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection;
    /// values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    /// As an example, you can pass {"50256": -100} to prevent the <|endoftext|> token from being generated.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i8>>,
    /// If set to true, the returned JSON will include a "prompt" field containing the final prompt that was used to request a completion.
    /// This is mainly useful for debugging purposes.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_prompt: Option<bool>,
    /// A special boolean flag for showing metadata. If set to true, each document entry in the returned JSON will contain a "metadata" field.
    /// This flag only takes effect when file is set.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_metadata: Option<bool>,
    /// If an object name is in the list, we provide the full information of the object;
    /// otherwise, we only provide the object ID. Currently we support completion and file objects for expansion.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<IntoVec<String>>,
    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// A response corresponding to a [`Request`]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// completion id
    pub completion: String,
    /// The chosen label for the query
    pub label: String,
    /// The model used for the completion of the request
    pub model: String,
    /// The requested action
    pub object: String,
    /// The model used for the search
    pub search_model: String,
    /// The examples used to judge the query
    pub selected_examples: Vec<SelectedExample>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedExample {
    /// The document the example is in
    pub document: usize,
    /// The label of the example
    pub label: String,
    /// The text of the example
    pub text: String,
    /// A list of the n most likely tokens
    pub logpropbs: Option<LogProbs>,
}

impl RequestInfo for Request {
    fn url(&self) -> String {
        format!("{OPENAI_URL}/classifications")
    }
}
#[cfg_attr(not(feature = "blocking"), async_trait::async_trait)]
impl crate::client::Request for Request {
    type Response = Response;
}
