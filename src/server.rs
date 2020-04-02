//! Spin up a HTTPServer

use crate::auth::get_identity_service;
// use crate::cache::add_cache;
use crate::config::CONFIG;
use crate::database::add_pool;
use crate::errors::ApiError;
use crate::handlers::error;
use crate::routes::routes;
use crate::state::new_state;
use actix_cors::Cors;
use actix_web::{
    error::JsonPayloadError,
    http,
    middleware::{errhandlers::ErrorHandlers, Logger},
    web::JsonConfig,
    App, HttpServer,
};
use listenfd::ListenFd;

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // Create the application state
    // String is used here, but it can be anything
    // Invoke in hanlders using data: AppState<'_, String>
    let data = new_state::<String>();

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        let error_handlers = ErrorHandlers::new()
            .handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                error::internal_server_error,
            )
            .handler(http::StatusCode::BAD_REQUEST, error::bad_request)
            .handler(http::StatusCode::NOT_FOUND, error::not_found);
        App::new()
            .app_data(
                JsonConfig::default().error_handler(|error, _req| match error {
                    JsonPayloadError::Overflow => {
                        ApiError::PayloadOverflow(error.to_string()).into()
                    }
                    _ => ApiError::JsonPayloadError(error.to_string()).into(),
                }),
            )
            // .configure(add_cache)
            .wrap(Cors::default())
            .wrap(Logger::default())
            .wrap(error_handlers)
            .wrap(get_identity_service())
            .configure(add_pool)
            .app_data(data.clone())
            .configure(routes)
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.server)?
    };

    server.run().await
}
