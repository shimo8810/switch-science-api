use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
    routing::get,
    Router,
};

use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{net::SocketAddr, time::Duration};

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// we can extract the connection pool with `Extension`
async fn using_connection_pool_extractor(
    Extension(pool): Extension<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect("postgres://postgres:password@localhost")
        .await?;

    let app = Router::new()
        .route("/", get(using_connection_pool_extractor))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
