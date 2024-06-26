use strum_macros::EnumString;

#[derive(EnumString, serde::Deserialize, Debug, Clone)]
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

#[derive(EnumString, serde::Deserialize, Debug, Clone)]
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

#[derive(EnumString, serde::Deserialize, Debug, Clone)]
pub enum OrderId {
    Int(i64),
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub quantity: String,
    pub price: String,
    pub timestamp: u64,
    pub side: OrderSide,
    pub kind: OrderKind,
    pub ttl: Option<u64>,
}
