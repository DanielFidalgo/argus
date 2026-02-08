use poem_openapi::{OpenApi, payload::PlainText};

pub struct HealthApi;

#[OpenApi]
impl HealthApi {
    /// Basic liveness endpoint.
    #[oai(path = "/healthz", method = "get")]
    async fn healthz(&self) -> PlainText<&'static str> {
        PlainText("ok")
    }

    /// Leapcell healthcheck endpoint.
    #[oai(path = "/kaithheathcheck", method = "get")]
    async fn kaithheathcheck(&self) -> PlainText<&'static str> {
        PlainText("ok")
    }

    /// Leapcell healthcheck endpoint.
    #[oai(path = "/kaithhealthcheck", method = "get")]
    async fn kaithhealthcheck(&self) -> PlainText<&'static str> {
        PlainText("ok")
    }
}
