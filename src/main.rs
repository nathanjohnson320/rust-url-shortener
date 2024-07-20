pub mod models;
pub mod schema;
pub mod urls;

use axum::{
    extract::{Path, State},
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    routing::{delete, post},
    Json, Router,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use models::{NewUrl, Url};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@localhost:5432/url_shortener".to_string());

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    // run the migrations on server startup
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    // build our application with a route
    let app = Router::new()
        .route("/urls", post(create_url).get(get_urls))
        .route("/urls/:id", delete(delete_url))
        .layer(
            CorsLayer::new()
                .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE])
                .allow_origin(tower_http::cors::Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::DELETE,
                    Method::PUT,
                    Method::OPTIONS,
                ]),
        )
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_url(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_url): Json<NewUrl>,
) -> Result<Json<Url>, (StatusCode, String)> {
    let conn = pool.get().await.unwrap();
    let res = conn
        .interact(move |conn| urls::create_url(conn, &new_url.long_url))
        .await
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn get_urls(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<Url>>, (StatusCode, String)> {
    let conn = pool.get().await.unwrap();
    let res = conn
        .interact(|conn| urls::get_urls(conn))
        .await
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn delete_url(
    Path(id): Path<i32>,
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Url>, (StatusCode, String)> {
    let conn = pool.get().await.unwrap();
    let res = conn
        .interact(move |conn| urls::delete_url(conn, &id))
        .await
        .map_err(internal_error)?;

    Ok(Json(res))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
