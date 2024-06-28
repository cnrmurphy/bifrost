use mongodb::{bson::doc, options::FindOptions};

use crate::{db::DB, models::Order};
use std::sync::{Arc, Mutex};

pub trait OrderRepo: Send + Sync {
    fn insert_order(&self, order: &Order);
    fn get_orders(&self) -> Result<Vec<Order>, Box<dyn std::error::Error>>;
}

pub struct InMemoryOrderRepo {
    orders: Arc<Mutex<Vec<Order>>>,
}

impl InMemoryOrderRepo {
    pub fn new() -> Self {
        Self {
            orders: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn new_with_arc() -> Arc<Self> {
        Arc::new(InMemoryOrderRepo::new())
    }
}

impl OrderRepo for InMemoryOrderRepo {
    fn insert_order(&self, order: &Order) {
        let mut orders = self.orders.lock().unwrap();
        orders.push(order.clone());
    }

    fn get_orders(&self) -> Result<Vec<Order>, Box<dyn std::error::Error>> {
        let orders = self.orders.lock().unwrap();
        Ok(orders.clone())
    }
}

#[derive(Debug, Clone)]
pub struct MongoOrderRepo {
    collection: mongodb::Collection<Order>,
}

impl MongoOrderRepo {
    pub fn new(db: &DB) -> Self {
        let collection = db.client.collection("orders");
        Self { collection }
    }

    pub async fn insert_order(
        &self,
        order: &Order,
    ) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error> {
        let result = self.collection.insert_one(order, None).await;
        return result;
    }

    pub async fn fetch_orders(&self) -> Result<Vec<Order>, mongodb::error::Error> {
        let mut orders: Vec<Order> = Vec::new();
        let options = FindOptions::builder().limit(100).build();
        let mut cursor = self.collection.find(None, options).await?;
        while cursor.advance().await? {
            let order = cursor.deserialize_current()?;
            orders.push(order);
        }

        Ok(orders)
    }
}
