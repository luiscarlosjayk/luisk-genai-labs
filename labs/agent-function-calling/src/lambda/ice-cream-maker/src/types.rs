/**
 * Reference:
 * - https://docs.aws.amazon.com/bedrock/latest/userguide/agents-lambda.html
 */
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//------------------- Request

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AgentRequest {
    pub message_version: String,
    pub agent: Agent,
    pub input_text: String,
    pub session_id: String,
    pub action_group: String,
    pub function: String,
    pub parameters: Vec<Parameters>,
    pub session_attributes: HashMap<String, String>,
    pub prompt_session_attributes: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub name: String,
    pub id: String,
    pub alias: String,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub name: String,
    pub value: String,
    pub r#type: ParameterType,
}

#[derive(Serialize, Debug)]
pub enum ParameterType {
    Text,
    Number,
    Integer,
    Boolean,
    Array,
    Unknown(String),
}

impl<'de> Deserialize<'de> for ParameterType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(ParameterType::from(s))
    }
}

impl From<String> for ParameterType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "string" => ParameterType::Text,
            "number" => ParameterType::Number,
            "integer" => ParameterType::Integer,
            "boolean" => ParameterType::Boolean,
            "array" => ParameterType::Array,
            unknown => ParameterType::Unknown(unknown.to_owned()),
        }
    }
}

//------------------- Response

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AgentResponse {
    pub message_version: String,
    pub response: Response,
    pub session_attributes: HashMap<String, String>,
    pub prompt_session_attributes: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub action_group: String,
    pub function: String,
    pub function_response: FunctionResponse,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FunctionResponse {
    #[serde(flatten)]
    pub response_type: ResponseType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ResponseType {
    State {
        #[serde(rename = "responseState")]
        response_state: ResponseState,
    },
    Body {
        #[serde(rename = "responseBody")]
        response_body: ResponseBody,
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged, rename_all = "UPPERCASE")]
pub enum ResponseState {
    Failure,
    Reprompt,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    #[serde(flatten)]
    pub content: HashMap<String, ContentType>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ContentType {
    pub body: String, // JSON-formatted string
}
