mod types;

use std::collections::HashMap;
use lambda_runtime::{
    run, service_fn,
    tracing::{self, instrument},
    Error, LambdaEvent,
};
use types::{AgentApiRequest, AgentApiResponse};

#[instrument(name = "waiter_handler", skip(event), fields(req_id = %event.context.request_id))]
async fn handler(event: LambdaEvent<AgentApiRequest>) -> Result<AgentApiResponse, Error> {
    tracing::info!("waiter handler invoked with payload: {:#?}", event);

    let agent_request = event.payload;
    let api_path = agent_request.api_path.as_str();
    let order_id = event.context.request_id;
    let mut response_content = HashMap::new();
    let mut http_status_code = 201;

    if api_path == "/start-order" {
        response_content.insert(
            "application/json".to_string(),
            types::ResponseBodyContentType {
                body: serde_json::json!({
                    "orderId": order_id
                }).to_string(),
            },
        );
    } else if api_path.starts_with("/add-ice-cream/") {
        response_content.insert(
            "application/json".to_string(),
            types::ResponseBodyContentType {
                body: serde_json::json!({
                    "message": format!("Ice cream added to order {order_id}"),
                }).to_string()
            },
        );
    } else if api_path.starts_with("/remove-ice-cream/") {
        http_status_code = 200;
        response_content.insert(
            "application/json".to_string(),
            types::ResponseBodyContentType {
                body: serde_json::json!({
                    "message": format!("Ice cream removed from order {order_id}"),
                }).to_string()
            },
        );
    } else {
        http_status_code = 500;
        response_content.insert(
            "application/json".to_string(),
            types::ResponseBodyContentType {
                body: serde_json::json!({
                    "message": "Unknown action",
                }).to_string()
            },
        );
    }

    let response = AgentApiResponse {
        message_version: "1.0".to_string(),
        session_attributes: agent_request.session_attributes,
        prompt_session_attributes: agent_request.prompt_session_attributes,
        response: types::ApiResponse {
            action_group: agent_request.action_group,
            api_path: agent_request.api_path,
            http_method: agent_request.http_method,
            http_status_code,
            response_body: types::ResponseBody {
                content: response_content,
            },
        },
    };

    tracing::info!("Response: {:?}", serde_json::to_string(&response)?);

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        // this needs to be set to remove duplicated information in the log.
        .with_current_span(false)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        // remove the name of the function from every log entry
        .with_target(false)
        .init();

    // let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let func = service_fn(handler);

    run(func).await
}
