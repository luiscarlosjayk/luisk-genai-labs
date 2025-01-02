/**
 * Reference:
 * - https://docs.aws.amazon.com/bedrock/latest/userguide/agents-lambda.html
 */
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//------------------- Request

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AgentApiRequest {
    pub message_version: String,
    pub agent: Agent,
    pub input_text: String,
    pub session_id: String,
    pub action_group: String,
    pub api_path: String,
    pub http_method: String,
    pub parameters: Vec<Parameters>,
    pub session_attributes: HashMap<String, String>,
    pub prompt_session_attributes: HashMap<String, String>,
    pub request_body: RequestBody,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Agent {
    pub name: String,
    pub id: String,
    pub alias: String,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    pub content: HashMap<String, RequestBodyContentType>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestBodyContentType {
    pub properties: Vec<Property>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Property {
    pub name: String,
    pub r#type: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
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
    D: serde::Deserializer<'de> {
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
pub struct AgentApiResponse {
   pub message_version: String,
   pub response: ApiResponse,
   pub session_attributes: HashMap<String, String>,
   pub prompt_session_attributes: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
   pub action_group: String,
   pub api_path: String,
   pub http_method: String,
   pub http_status_code: i32,
   pub response_body: ResponseBody,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
   #[serde(flatten)]
   pub content: HashMap<String, ResponseBodyContentType>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBodyContentType {
   pub body: String,  // JSON-formatted string
}
