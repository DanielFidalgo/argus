use poem::{IntoEndpoint, Route, endpoint::StaticFilesEndpoint, get, handler, web::Redirect};
use poem_openapi::OpenApiService;

use crate::application::routes::health_check::HealthApi;

pub mod admin;
pub mod health_check;

pub fn routes() -> Route {
    let api_service = OpenApiService::new(HealthApi, "Argus API", "0.1.0").server("/");
    let asset_files = StaticFilesEndpoint::new("assets");
    let static_files = StaticFilesEndpoint::new("static"); // paths are absolute; keep server root

    Route::new()
        // exposes /healthz and /kaithheathcheck
        .nest("/", get(root))
        .nest("/docs", api_service.scalar()) // Swagger UI at /docs
        .nest("/openapi", api_service.spec_endpoint())
        .nest("/api", api_service.into_endpoint())
        .nest("/admin", admin::routes())
        .nest("/assets", asset_files)
        .nest("/static", static_files)
}

#[handler]
fn root() -> Redirect {
    Redirect::permanent("/admin")
}
