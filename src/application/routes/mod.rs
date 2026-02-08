use poem::{IntoEndpoint, Route};
use poem_openapi::OpenApiService;

use crate::application::routes::health_check::HealthApi;

pub mod health_check;

pub fn routes() -> Route {
    let api_service = OpenApiService::new(HealthApi, "Argus API", "0.1.0").server("/"); // paths are absolute; keep server root

    Route::new()
        // exposes /healthz and /kaithheathcheck
        .nest("/docs", api_service.scalar()) // Swagger UI at /docs
        .nest("/openapi", api_service.spec_endpoint())
        .nest("/", api_service.into_endpoint()) // OpenAPI spec JSON at /api
}
