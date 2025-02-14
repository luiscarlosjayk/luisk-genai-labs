#[instrument(name = "stub_handler", fields(req_id = %event.context.request_id))]
pub async fn handler(event: LambdaEvent<serde_json::Value>) -> Result<(), Error> {
    tracing::info!("stub handler invoked");

    Ok(())
}
