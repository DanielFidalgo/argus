use askama::Template;
use askama_web::WebTemplate;
use poem::handler;

#[derive(Template, WebTemplate)]
#[template(path = "admin.html")]
struct AdminTmpl;

#[handler]
pub async fn admin_handler() -> AdminTmpl {
    AdminTmpl
}
