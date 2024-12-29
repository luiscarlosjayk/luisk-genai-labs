use lambda_runtime::{
    run, service_fn,
    tracing::{self, instrument},
    Error, LambdaEvent,
};

#[instrument(name = "stub_handler", fields(req_id = %event.context.request_id))]
async fn handler(
    event: LambdaEvent<S3Event>,
) -> Result<(), Error> {
    tracing::info!("stub handler invoked");

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
    let func = service_fn(|event| handler(event));

    run(func).await
}
