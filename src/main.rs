#![feature(derive_default_enum)]
#![feature(crate_visibility_modifier)]

use crate::{client::Client, api::{answers, classifications, completions}};

pub mod api;
pub mod client;
mod model;

const OPENAI_URL: &str = "https://api.openai.com/v1";
fn main() {
    println!("Hello, world!");
    let client = Client::new(String::from("aaaa"));
    //client.request(completions::Request::builder().prompt(String::from("test")));
}
