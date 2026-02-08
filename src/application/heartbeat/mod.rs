use std::time::Duration;

use tokio::{task::JoinHandle, time::sleep};

use crate::configuration::service_setup::ServiceError;

#[allow(dead_code)]
pub fn spawn() -> JoinHandle<Result<(), ServiceError>> {
    tokio::spawn(async move {
        tracing::info!("Argus heartbeat started");
        loop {
            tracing::info!("Hello world from Argus!");
            sleep(Duration::from_secs(300)).await; // 5 minutes
        }

        #[allow(unreachable_code)]
        Ok(())
    })
}
