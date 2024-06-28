use mongodb::bson::oid::{self, ObjectId};
use strum_macros::EnumString;

#[derive(EnumString, serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl std::fmt::Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        tracing::debug!("writing order side");
        match self {
            OrderSide::Buy => write!(f, "Buy"),
            OrderSide::Sell => write!(f, "Sell"),
        }
    }
}

#[derive(EnumString, serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum OrderKind {
    Market,
    Limit,
}

impl std::fmt::Display for OrderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        tracing::debug!("writing order kind");
        match self {
            OrderKind::Limit => write!(f, "Limit"),
            OrderKind::Market => write!(f, "Market"),
        }
    }
}

#[derive(EnumString, serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum OrderId {
    Int(i64),
}

#[derive(EnumString, serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum OrderStatus {
    Open,
    Filled,
    Cancelled,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        tracing::debug!("writing order kind");
        match self {
            OrderStatus::Open => write!(f, "Open"),
            OrderStatus::Filled => write!(f, "Filled"),
            OrderStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Order {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub symbol: String,
    pub quantity: String,
    pub price: String,
    pub side: OrderSide,
    pub kind: OrderKind,
    pub status: Option<OrderStatus>,
    pub ttl: Option<u64>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct OrderRequest {
    pub symbol: String,
    pub quantity: String,
    pub price: String,
    pub side: OrderSide,
    pub kind: OrderKind,
    pub ttl: Option<u64>,
}

impl Into<Order> for OrderRequest {
    fn into(self) -> Order {
        Order {
            id: oid::ObjectId::new(),
            symbol: self.symbol,
            quantity: self.quantity,
            price: self.price,
            side: self.side,
            kind: self.kind,
            status: Some(OrderStatus::Open),
            ttl: None,
        }
    }
}
