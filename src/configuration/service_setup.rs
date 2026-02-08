use std::{future::Future, pin::Pin, time::Instant};

use poem::{EndpointExt, IntoEndpoint, middleware::Tracing};
use tokio::task::{JoinError, JoinHandle, JoinSet};

pub struct Config<E>
where
    E: IntoEndpoint,
    E::Endpoint: 'static,
{
    pub service_url: String,
    pub port: u16,
    pub routes: E,
}

pub type HandlerFn = JoinHandle<Result<(), ServiceError>>;
pub type TeardownFn = Box<dyn FnOnce() -> TeardownFuture + Send>;
pub type TeardownFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("poem error: {0}")]
    Poem(#[from] poem::Error),

    #[error("signal handler error: {0}")]
    Signal(#[source] std::io::Error),

    #[error("{task} failed: {source}")]
    TaskFailed {
        task: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("{task} join failed: {source}")]
    TaskJoin {
        task: String,
        #[source]
        source: tokio::task::JoinError,
    },
}

pub async fn service_setup<E>(
    config: Config<E>,
    mut handlers: Vec<(String, HandlerFn)>,
    teardown: Vec<TeardownFn>,
) -> Result<(), ServiceError>
where
    E: IntoEndpoint,
    E::Endpoint: 'static,
{
    handlers.push(("Server".into(), server_setup(config)));
    handlers.push(("Signals".into(), signals()));

    tracing::info!(tasks = handlers.len(), "starting service");

    // Run tasks concurrently; stop when the first completes (success or failure).
    let start = Instant::now();
    let (task, join_res) = first_finished(handlers).await;
    tracing::info!(%task, secs = start.elapsed().as_secs(), "task finished");

    match join_res {
        Ok(Ok(())) => tracing::info!(%task, "task completed"),
        Ok(Err(e)) => {
            return Err(ServiceError::TaskFailed {
                task,
                source: Box::new(e),
            });
        }
        Err(e) => return Err(ServiceError::TaskJoin { task, source: e }),
    }

    tracing::info!(teardown = teardown.len(), "running teardown");
    for f in teardown {
        f().await;
    }

    Ok(())
}

async fn first_finished(
    handlers: Vec<(String, HandlerFn)>,
) -> (String, Result<Result<(), ServiceError>, JoinError>) {
    let mut set = JoinSet::new();

    for (task, handle) in handlers {
        set.spawn(async move { (task, handle.await) });
    }

    // If there are no tasks, return a no-op completion.
    set.join_next()
        .await
        .map(|r| match r {
            Ok(v) => v,
            Err(e) => ("service-wrapper".into(), Err(e)),
        })
        .unwrap_or_else(|| ("no-tasks".into(), Ok(Ok(()))))
}

pub fn make_teardown<F, Fut>(f: F) -> TeardownFn
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    Box::new(move || Box::pin(f()))
}

pub fn server_setup<E>(config: Config<E>) -> HandlerFn
where
    E: IntoEndpoint,
    E::Endpoint: Send + 'static,
{
    // Destructure so we don't move `E` into the spawned task.
    let Config {
        service_url,
        port,
        routes,
    } = config;

    // Convert to endpoint on the current thread.
    let app = routes.into_endpoint().with(Tracing);

    tokio::spawn(async move {
        tracing::info!("Server is running on {}", service_url);

        poem::Server::new(poem::listener::TcpListener::bind(format!("0.0.0.0:{port}")))
            .run(app)
            .await
            .map_err(ServiceError::from)
    })
}

pub fn signals() -> HandlerFn {
    tokio::spawn(async move {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{SignalKind, signal};

            let mut sigterm = signal(SignalKind::terminate()).map_err(ServiceError::Signal)?;
            let mut sigint = signal(SignalKind::interrupt()).map_err(ServiceError::Signal)?;
            let mut sigquit = signal(SignalKind::quit()).map_err(ServiceError::Signal)?;

            tokio::select! {
                _ = sigterm.recv() => tracing::info!("Received SIGTERM"),
                _ = sigint.recv()  => tracing::info!("Received SIGINT"),
                _ = sigquit.recv() => tracing::info!("Received SIGQUIT"),
            }
        }

        #[cfg(not(unix))]
        {
            tokio::signal::ctrl_c().await.map_err(|e| {
                ServiceError::Signal(std::io::Error::new(std::io::ErrorKind::Other, e))
            })?;
            tracing::info!("Received Ctrl-C");
        }

        Ok(())
    })
}
