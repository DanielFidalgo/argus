use poem::{IntoEndpoint, Route};
use poem_openapi::OpenApiService;

use crate::application::routes::{admin::admin_handler, health_check::HealthApi};

pub mod admin;
pub mod health_check;
pub mod partials;

pub fn routes() -> Route {
    let api_service = OpenApiService::new(HealthApi, "Argus API", "0.1.0").server("/"); // paths are absolute; keep server root

    Route::new()
        // exposes /healthz and /kaithheathcheck
        .nest("/docs", api_service.scalar()) // Swagger UI at /docs
        .nest("/openapi", api_service.spec_endpoint())
        .nest("/", api_service.into_endpoint())
        .nest(
            "/admin",
            Route::new()
                .at("/", poem::get(admin_handler))
                .nest("partials", partials::routes()),
        )
}
