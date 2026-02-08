use crate::{
    application::{heartbeat, routes::routes},
    configuration::service_setup::{Config, HandlerFn, make_teardown, service_setup},
};

mod application;
mod configuration;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), configuration::service_setup::ServiceError> {
    tracing_subscriber::fmt::init();

    let routes = routes();

    let config = Config {
        service_url: "http://0.0.0.0:8080".to_string(),
        port: 8080,
        routes,
    };

    let handlers: Vec<(String, HandlerFn)> = vec![("Heartbeat".into(), heartbeat::spawn())];

    let teardown = vec![make_teardown(|| async move {
        tracing::info!("teardown done");
    })];

    service_setup(config, handlers, teardown).await
}
