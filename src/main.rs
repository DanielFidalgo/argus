use std::time::Duration;

use tokio::time::sleep;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(FmtSubscriber::builder().finish())
        .expect("setting default subscriber failed");
    info!("Argus agent starting...");

    loop {
        info!("Hello world from Argus!");
        sleep(Duration::from_secs(300)).await; // 5 minutes
    }
}
