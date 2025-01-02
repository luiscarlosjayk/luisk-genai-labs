mod types;

use aws_sdk_bedrockagentruntime::types::{
    ActionGroupExecutor, AgentActionGroup, FunctionDefinition, FunctionSchema,
    InlineAgentResponseStream, ParameterDetail, ParameterType, ApiSchema,
};
use lambda_runtime::{
    run, service_fn,
    tracing::{self, instrument},
    Error, LambdaEvent,
};
use types::ClientPrompt;

/// The instructions that tell the inline agent what it should do and how it should interact with users.
const AGENT_INSTRUCTION: &str = r#"
    You are an ice cream making assistant in charge of operating the ice cream machine. Your role is to:

    1. Accept a new order with given client's name and create a new order based on it.
    2. Accept ice cream flavor requests and:
    2.1. Add flavors to a given order if requested.
    2.2. Remove flavors to a given order if requested.
    2.3. Prepare the ice creams of the falvors in it.
    2. Understand different ways customers might request flavors (e.g., "vanilla", "chocolate chip", "strawberry").
    3. Respond appropriately to requests, including:
        - Confirming when an ice cream has been prepared.
        - Explaining if a requested flavor isn't available.
        - Rejecting the request if the flavor request is ambiguous.
        - Handling one flavor request at a time.

    Extra Guidelines:
    - You can prepare these flavors:
        1. Vanilla
        2. Chocolate
        3. Strawberry
        4. Mint Chocolate Chip
        5. Cookie Dough
    - You can take up to 5 flavors on a given order, if there are more the order should be split.
    - When someone requests a flavor not on this list, explain which flavors are available instead.
    - You cannot prepare ice cream flavors if you haven't added those flavors to the given order first.

    Tone: Always maintain a friendly, helpful tone while focusing on the core task of ice cream preparation.
"#;

#[instrument(name = "agent_caller_handler", skip(event, bedrock_agentruntime_client), fields(req_id = %event.context.request_id))]
async fn handler(
    event: LambdaEvent<ClientPrompt>,
    bedrock_agentruntime_client: &aws_sdk_bedrockagentruntime::Client,
) -> Result<(), Error> {
    tracing::info!("AgentCaller handler invoked with payload: {:#?}", event);

    let request_id = event.context.request_id;
    let input_prompt = event.payload.input;
    let enable_trace = std::env::var("ENABLE_TRACE")
        .map(|val| val.parse::<bool>().unwrap_or(true))
        .unwrap_or(true);
    let Ok(model_id) = std::env::var("MODEL_ID") else {
        return Err(r#"Missing "MODEL_ID" environment variable."#.into());
    };

    // Action: Ice Cream Maker
    let Ok(ice_cream_maker_lambda) = std::env::var("ICE_CREAM_MAKER_LAMBDA") else {
        return Err(r#"Missing "ICE_CREAM_MAKER_LAMBDA" environment variable "#.into());
    };
    let ice_cream_maker_action_group =
        AgentActionGroup::builder()
            .action_group_name("IceCreamMaker")
            .action_group_executor(ActionGroupExecutor::Lambda(ice_cream_maker_lambda))
            .description(
                r"
                    ActionGroup that allows to manage the ice cream maker to create ice creams.
                ",
            )
            .function_schema(FunctionSchema::Functions(vec![
            FunctionDefinition::builder()
                .name("PrepareIceCream")
                .description(
                    r"
                        Manages the ice cream maker machine to make the ice creams of a given flavor.
                        Should make sense as a realistic flavor for an ice cream.
                        Once invoked, aknowledge it takes almost zero seconds for the ice cream to be made.
                    ",
                )
                .parameters("flavor", ParameterDetail::builder()
                .description("Flavor requested to be made with the ice cream maker machine.")
                .r#type(ParameterType::String)
                .required(true)
                .build()?)
                .build()?,
        ]))
            .build()?;

    // Action: Waiter
    let Ok(waiter_lambda) = std::env::var("WAITER_LAMBDA") else {
        return Err(r#"Missing "WAITER_LAMBDA" environment variable "#.into());
    };
    let waiter_api_schema_content = include_str!("../../waiter/schemas/waiter.yaml");
    let waiter_api_schema = ApiSchema::Payload(waiter_api_schema_content.to_string());
    let waiter_action_group = AgentActionGroup::builder()
            .action_group_name("Waiter")
            .action_group_executor(ActionGroupExecutor::Lambda(waiter_lambda))
            .description(
                r"
                    ActionGroup that helps taking orders, adding and removing flavors from it.
                ",
            )
            .api_schema(waiter_api_schema)
        .build()?;

    // Agent Invoke Inline
    let invoke_response = bedrock_agentruntime_client
        .invoke_inline_agent()
        .session_id(request_id)
        .enable_trace(enable_trace)
        .input_text(input_prompt)
        .foundation_model(model_id)
        .instruction(AGENT_INSTRUCTION)
        .action_groups(ice_cream_maker_action_group)
        .action_groups(waiter_action_group)
        .send()
        .await?;

    let mut response = invoke_response.completion;

    // Consume chunks from the stream
    while let Some(event) = response.recv().await? {
        match event {
            InlineAgentResponseStream::Chunk(chunk) => {
                if let Some(response_chunk) = chunk.bytes {
                    let blob_text = String::from_utf8(response_chunk.into_inner())?;
                    tracing::info!("Response Chunk: {:?}", &blob_text);
                }
            }
            InlineAgentResponseStream::Trace(inline_agent_trace_part) => {
                if let Some(trace_chunk) = inline_agent_trace_part.trace {
                    tracing::info!("Trace chunk: {:?}", trace_chunk);
                }
            }
            _ => {
                // Do nothing
            }
        }
    }

    // tracing::info!("Response: {:#?}", response);

    Ok(())
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

    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let bedrock_agentruntime_client = aws_sdk_bedrockagentruntime::Client::new(&sdk_config);
    let func = service_fn(|event| handler(event, &bedrock_agentruntime_client));

    run(func).await
}