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
) -> Result<(), Error> {
    tracing::info!("handler invoked");

    let model_arn = std::env::var("MODEL_ARN").unwrap();
    let records = event.payload.records;
    let first_record = records.first().unwrap();
    let bucket_name = first_record.s3.bucket.name.as_ref().unwrap();
    let object_key = first_record.s3.object.key.as_ref().unwrap();

    tracing::info!({ %bucket_name, object_key }, "First record retrieved.");

    let input = RetrieveAndGenerateInput::builder()
        .text(PROMPT)
        .build()
        .unwrap();
    let retrieve_and_generate_type = RetrieveAndGenerateType::ExternalSources;
    let s3_document_uri = format!("s3://{bucket_name}/{object_key}");
    let s3_retrieval_doc = S3ObjectDoc::builder()
        .set_uri(Some(s3_document_uri))
        .build()
        .unwrap();
    let retrieval_s3_sources = ExternalSource::builder()
        .set_source_type(Some(ExternalSourceType::S3))
        .set_s3_location(Some(s3_retrieval_doc))
        .build()
        .unwrap();
    let external_source_retrieval_configuration =
        ExternalSourcesRetrieveAndGenerateConfiguration::builder()
            .set_model_arn(Some(model_arn))
            .set_sources(Some(vec![retrieval_s3_sources]))
            .build()
            .unwrap();
    let retrieve_and_generate_configuration = RetrieveAndGenerateConfiguration::builder()
        .set_type(Some(retrieve_and_generate_type))
        .set_external_sources_configuration(Some(external_source_retrieval_configuration))
        .build()
        .unwrap();

    let response = bedrock_agent_runtime_client
        .retrieve_and_generate()
        .input(input)
        .set_retrieve_and_generate_configuration(Some(retrieve_and_generate_configuration))
        .send()
        .await?;
    let response_output = response.output().unwrap().text();

    tracing::info!("Response: {:#?}", response_output);

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
