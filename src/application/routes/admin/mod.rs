use crate::application::routes::admin::pages::{admin_handler, home, scanner, status};

pub mod pages;

pub fn routes() -> poem::Route {
    poem::Route::new()
        .at("/", poem::get(admin_handler))
        .at("/pages/home", poem::get(home))
        .at("/pages/status", poem::get(status))
        .at("/pages/scanner", poem::get(scanner))
}
