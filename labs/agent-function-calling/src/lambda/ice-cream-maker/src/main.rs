mod types;

use std::collections::HashMap;

use lambda_runtime::{
    run, service_fn,
    tracing::{self, instrument},
    Error, LambdaEvent,
};
use types::{AgentRequest, AgentResponse};

#[instrument(name = "ice_cream_maker", skip(event), fields(req_id = %event.context.request_id))]
async fn handler(event: LambdaEvent<AgentRequest>) -> Result<AgentResponse, Error> {
    tracing::info!(
        "IceCreamMaker handler invoked with payload: {:?}",
        event.payload
    );

    let agent_request = event.payload;
    let Some(flavor_parameter) = agent_request.parameters.first() else {
        return Err("Expected at least one parameter defined. None found.".into());
    };
    let flavor = flavor_parameter.value.clone();

    tracing::info!("Preparing ice cream of {} flavor", flavor);

    let mut response_content = HashMap::new();
    response_content.insert(
        "TEXT".to_string(),
        types::ContentType {
            body: format!("Ice cream of {flavor} was made."),
        },
    );

    let response = AgentResponse {
        message_version: "1.0".to_string(),
        session_attributes: agent_request.session_attributes,
        prompt_session_attributes: agent_request.prompt_session_attributes,
        response: types::Response {
            action_group: agent_request.action_group,
            function: agent_request.function,
            function_response: types::FunctionResponse {
                response_type: types::ResponseType::Body {
                    response_body: types::ResponseBody {
                        content: response_content,
                    },
                },
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

    let func = service_fn(handler);

    run(func).await
}
