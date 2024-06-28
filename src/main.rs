mod db;
mod models;
mod repository;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use models::{Order, OrderRequest};
use repository::{InMemoryOrderRepo, MongoOrderRepo, OrderRepo};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Template)]
#[template(
    source = r#"
    <h1>Portfolio</h1>
    <ul>
    {% for order in orders %}
        <li>{{ order.symbol }} - {{ order.side }} - {{ order.kind }} - {{ order.quantity }} - {% if order.status.is_some() %} {{ order.status.as_ref().unwrap() }} {% endif %}</li>
    {% endfor %}
    <ul>
    "#,
    ext = "txt"
)]
struct OrdersTemplate<'a> {
    orders: &'a Vec<Order>,
}

#[derive(Clone)]
struct AppState {
    order_repo: Arc<dyn OrderRepo>,
    mongo_order_repo: MongoOrderRepo,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_key_value_store=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = db::DB::init().await.unwrap();

    let order_repo = InMemoryOrderRepo::new_with_arc();
    let mongo_order_repo = MongoOrderRepo::new(&db);

    let app = Router::new()
        .route("/", get(root))
        .route("/portfolio", get(render_portfolio))
        .route("/orders/limit", post(place_limit_order))
        .route("/orders/market", post(place_market_order))
        .with_state(AppState {
            order_repo,
            mongo_order_repo,
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Bifrost listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello world!"
}

async fn render_portfolio(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let orders = state
        .mongo_order_repo
        .fetch_orders()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tmpl = OrdersTemplate { orders: &orders };
    tmpl.render()
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn place_limit_order(
    State(state): State<AppState>,
    Json(input): Json<OrderRequest>,
) -> StatusCode {
    tracing::debug!("limit order request");
    tracing::debug!("{:?}", input);
    let result = state.mongo_order_repo.insert_order(&input.into()).await;
    match result {
        Ok(result) => {
            tracing::debug!("{:?}", result);
            StatusCode::OK
        }
        Err(error) => {
            tracing::error!("{:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn place_market_order(
    State(state): State<AppState>,
    Json(input): Json<OrderRequest>,
) -> StatusCode {
    tracing::debug!("market order request");
    tracing::debug!("{:?}", input);
    let result = state.mongo_order_repo.insert_order(&input.into()).await;
    match result {
        Ok(result) => {
            tracing::debug!("{:?}", result);
            StatusCode::OK
        }
        Err(error) => {
            tracing::error!("{:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
