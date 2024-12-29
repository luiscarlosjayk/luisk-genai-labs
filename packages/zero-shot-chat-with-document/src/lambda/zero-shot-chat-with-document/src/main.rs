use aws_lambda_events::event::s3::S3Event;
use aws_sdk_bedrockagentruntime::types::{
    ExternalSource, ExternalSourceType, ExternalSourcesRetrieveAndGenerateConfiguration,
    RetrieveAndGenerateConfiguration, RetrieveAndGenerateInput, RetrieveAndGenerateType,
    S3ObjectDoc,
};
use lambda_runtime::{
    run, service_fn,
    tracing::{self, instrument},
    Error, LambdaEvent,
};

/// Prompt used to query the foundational model
const PROMPT: &str = "Summarize in few sentences the given document.";

#[instrument(name = "zero_shot_chat_with_document_handler", skip(bedrock_agent_runtime_client), fields(req_id = %event.context.request_id))]
async fn handler(
    event: LambdaEvent<S3Event>,
    bedrock_agent_runtime_client: &aws_sdk_bedrockagentruntime::Client,
) -> Result<String, Error> {
    tracing::info!("handler invoked");

    let model_arn =
        std::env::var("MODEL_ARN").expect("Expected MODEL_ARN environment variable to be defined");
    let records = event.payload.records;
    let Some(first_record) = records.first() else {
        return Err("No record to process in event received".into());
    };
    let Some(bucket_name) = first_record.s3.bucket.name.as_ref() else {
        return Err("No bucket name found in record to process".into());
    };
    let Some(object_key) = first_record.s3.object.key.as_ref() else {
        return Err("".into());
    };

    tracing::info!({ %bucket_name, object_key }, "First record retrieved.");

    let input = RetrieveAndGenerateInput::builder().text(PROMPT).build()?;
    let retrieve_and_generate_type = RetrieveAndGenerateType::ExternalSources;
    let s3_document_uri = format!("s3://{bucket_name}/{object_key}");
    let s3_retrieval_doc = S3ObjectDoc::builder()
        .set_uri(Some(s3_document_uri))
        .build()?;
    let retrieval_s3_sources = ExternalSource::builder()
        .set_source_type(Some(ExternalSourceType::S3))
        .set_s3_location(Some(s3_retrieval_doc))
        .build()?;
    let external_source_retrieval_configuration =
        ExternalSourcesRetrieveAndGenerateConfiguration::builder()
            .set_model_arn(Some(model_arn))
            .set_sources(Some(vec![retrieval_s3_sources]))
            .build()?;
    let retrieve_and_generate_configuration = RetrieveAndGenerateConfiguration::builder()
        .set_type(Some(retrieve_and_generate_type))
        .set_external_sources_configuration(Some(external_source_retrieval_configuration))
        .build()?;

    let response = bedrock_agent_runtime_client
        .retrieve_and_generate()
        .input(input)
        .set_retrieve_and_generate_configuration(Some(retrieve_and_generate_configuration))
        .send()
        .await?;
    let response_output = response
        .output()
        .ok_or("Expected text output defined from RetrieveAndGenerate response. Got None instead.")?
        .text();

    tracing::info!("Response: {:#?}", response_output);

    Ok(response_output.into())
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
