mod models;
mod repository;

use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use models::Order;
use repository::{InMemoryOrderRepo, OrderRepo};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Template)]
#[template(
    source = r#"
    <h1>Portfolio</h1>
    <ul>
    {% for order in orders %}
        <li>{{ order.symbol }} - {{ order.side }} - {{ order.kind }} - {{ order.quantity }}</li>
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
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_key_value_store=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let order_repo = InMemoryOrderRepo::new_with_arc();

    let app = Router::new()
        .route("/", get(root))
        .route("/portfolio", get(render_portfolio))
        .route("/orders/limit", post(place_limit_order))
        .route("/orders/market", post(place_market_order))
        .with_state(AppState { order_repo });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Bifrost listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello world!"
}

async fn render_portfolio(State(state): State<AppState>) -> Html<String> {
    let orders = state.order_repo.get_orders().unwrap();
    let tmpl = OrdersTemplate { orders: &orders };
    Html(tmpl.render().unwrap())
}

async fn place_limit_order(State(state): State<AppState>, Json(input): Json<Order>) -> StatusCode {
    tracing::debug!("limit order request");
    tracing::debug!("{:?}", input);
    state.order_repo.add_order(&input);

    StatusCode::OK
}

async fn place_market_order(State(state): State<AppState>, Json(input): Json<Order>) -> StatusCode {
    tracing::debug!("market order request");
    tracing::debug!("{:?}", input);
    state.order_repo.add_order(&input);

    StatusCode::OK
}
