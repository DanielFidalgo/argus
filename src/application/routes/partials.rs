use askama::Template;
use askama_web::WebTemplate;
use poem::handler;

#[derive(Template, WebTemplate)]
#[template(path = "partials/home.html")]
struct HomeTmpl;

// partial status
#[derive(Template, WebTemplate)]
#[template(path = "partials/status.html")]
struct StatusTmpl {
    heartbeat: String,
    service: String,
}

#[handler]
pub async fn home() -> HomeTmpl {
    HomeTmpl
}

#[handler]
pub async fn status() -> StatusTmpl {
    StatusTmpl {
        heartbeat: "OK".to_string(),
        service: "Argus".to_string(),
    }
}

pub fn routes() -> poem::Route {
    poem::Route::new()
        .at("/home", poem::get(home))
        .at("/status", poem::get(status))
}
