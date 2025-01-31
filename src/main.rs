use lambda_extension::{service_fn, tracing, Error, Extension};

mod events_extension;
use events_extension::events_processor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    Extension::new()
        .with_events_processor(service_fn(events_processor))
        .run()
        .await
}
