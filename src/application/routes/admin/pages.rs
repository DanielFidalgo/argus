use askama::Template;
use askama_web::WebTemplate;
use chrono::{Datelike, Local};
use poem::{IntoResponse, Request, Response, handler, http::StatusCode};

/// navigation item
pub struct SidebarItem {
    pub label: &'static str,
    pub href: &'static str,
    pub icon: &'static str,
}

#[allow(dead_code)]
pub struct NavbarItem {
    pub label: &'static str,
    pub href: &'static str,
    pub icon: &'static str,
}

/// global template context
#[allow(dead_code)]
pub struct BaseTemplateContext {
    year: i32,
    sidebar_items: Vec<SidebarItem>,
    navbar_items: Vec<NavbarItem>,
}

/// build shared context from request
fn make_base_context() -> BaseTemplateContext {
    let now = Local::now();

    // nav list
    let sidebar_items = vec![
        SidebarItem {
            label: "Home",
            href: "/admin/pages/home",
            icon: "mdi-home",
        },
        SidebarItem {
            label: "Status",
            href: "/admin/pages/status",
            icon: "mdi-desktop-mac",
        },
    ];

    let navbar_items = vec![
        NavbarItem {
            label: "Home",
            href: "/admin/pages/home",
            icon: "mdi-home",
        },
        NavbarItem {
            label: "Status",
            href: "/admin/pages/status",
            icon: "mdi-desktop-mac",
        },
    ];

    BaseTemplateContext {
        year: now.year(),
        sidebar_items,
        navbar_items,
    }
}

/// Check if request is from HTMX
fn is_htmx_request(req: &Request) -> bool {
    req.headers()
        .get("HX-Request")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "true")
        .unwrap_or(false)
}

//
// === Askama Templates ===
//

/// admin layout page
#[derive(Template, WebTemplate)]
#[template(path = "pages/admin.html")]
pub struct AdminTmpl {
    base: BaseTemplateContext,
}

/// home page - full
#[derive(Template, WebTemplate)]
#[template(path = "pages/home.html")]
pub struct HomeTmpl {
    _base: BaseTemplateContext,
}

/// home page - partial
#[derive(Template, WebTemplate)]
#[template(path = "pages/home_partial.html")]
pub struct HomePartialTmpl {}

/// status page - full
#[derive(Template, WebTemplate)]
#[template(path = "pages/status.html")]
pub struct StatusTmpl {
    _base: BaseTemplateContext,
    heartbeat: String,
    service: String,
}

/// status page - partial
#[derive(Template, WebTemplate)]
#[template(path = "pages/status_partial.html")]
pub struct StatusPartialTmpl {
    heartbeat: String,
    service: String,
}

//
// === Handlers ===
//

#[handler]
pub async fn admin_handler(_req: &Request) -> impl IntoResponse {
    let base = make_base_context();
    AdminTmpl { base }
}

#[handler]
pub async fn home(req: &Request) -> Response {
    if is_htmx_request(req) {
        // HTMX request - return only content partial
        let template = HomePartialTmpl {};
        match template.render() {
            Ok(html) => Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html; charset=utf-8")
                .body(html),
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Template error: {}", e)),
        }
    } else {
        // Direct access - return full page with layout
        let template = HomeTmpl {
            _base: make_base_context(),
        };
        match template.render() {
            Ok(html) => Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html; charset=utf-8")
                .body(html),
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Template error: {}", e)),
        }
    }
}

#[handler]
pub async fn status(req: &Request) -> Response {
    let heartbeat = "OK".to_string();
    let service = "Argus".to_string();

    if is_htmx_request(req) {
        // HTMX request - return only content partial
        let template = StatusPartialTmpl {
            heartbeat: heartbeat.clone(),
            service: service.clone(),
        };
        match template.render() {
            Ok(html) => Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html; charset=utf-8")
                .body(html),
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Template error: {}", e)),
        }
    } else {
        // Direct access - return full page with layout
        let template = StatusTmpl {
            _base: make_base_context(),
            heartbeat,
            service,
        };
        match template.render() {
            Ok(html) => Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html; charset=utf-8")
                .body(html),
            Err(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Template error: {}", e)),
        }
    }
}
