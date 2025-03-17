use rs_party::db::get_pool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
pub async fn main() {
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
    let migration_result = sqlx::migrate!("./src/sql/migrations").run(&db_pool).await;

    match migration_result {
        Ok(_) => {
            tracing::info!("Migration complete")
        }
        Err(e) => {
            tracing::error!("Failed to run migrations: {}", e.to_string())
        }
    };
}
