//! Main module

use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;

use tracing::{event, info_span, Level, Span};

use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
    routing::{delete, get, post},
    Router,
};
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::{DefaultOnRequest, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rs_party::{
    db::get_pool,
    middleware::create_cors_layer,
    model::RequestLogEntry,
    routes::{self, get_hc_handler, login_handler, registration_handler, AppState},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_pool_result = get_pool().await;

    let db_pool = match db_pool_result {
        Ok(pool) => {
            tracing::info!("DB Connection Succeeded");
            pool
        }
        Err(e) => {
            panic!("Could not connect to database\n{:?}", e);
        }
    };

    let app_state = Arc::new(AppState { db: db_pool });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let app = Router::new()
        .route("/", get(get_hc_handler))
        .route("/hc", get(get_hc_handler))
        .route("/login", post(login_handler))
        .route("/register", post(registration_handler))
        .route("/user/self", get(routes::get_user_self_handler))
        .route(
            "/event",
            post(routes::post_event_handler).patch(routes::patch_event_handler),
        )
        .route("/event/{*event_id}", delete(routes::delete_event_handler))
        .route("/event/own", get(routes::get_owned_events_handler))
        .with_state(app_state)
        .layer(create_cors_layer())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                        time_received = Utc::now().to_string(),
                    )
                })
                .on_request(
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                    DefaultOnRequest::new().level(Level::INFO),
                )
                .on_response(|_response: &Response, _latency: Duration, span: &Span| {
                    span.record("time_responded", Utc::now().to_string());

                    let time_logged_str = match span.field("time_received") {
                        Some(field) => field.to_string(),
                        None => "".to_string(),
                    };

                    let method_str = match span.field("method") {
                        Some(method_field) => method_field.to_string(),
                        None => "".to_string(),
                    };

                    let path_str = match span.field("matched_path") {
                        Some(path_field) => path_field.to_string(),
                        None => "/".to_string(),
                    };

                    let _entry = RequestLogEntry {
                        id: None,
                        time_received: time_logged_str,
                        time_logged: Utc::now().to_string(),
                        method: method_str,
                        req_headers: "".to_string(),
                        req_url: path_str,
                    };

                    // db::insert_log_entry(&mut log_conn, entry);

                    event!(parent: span, Level::INFO, "finished processing request")
                    // ...
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        );

    axum::serve(listener, app).await.unwrap();
}
