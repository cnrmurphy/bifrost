use crate::models::Order;
use std::sync::{Arc, Mutex};

pub trait OrderRepo: Send + Sync {
    fn add_order(&self, order: &Order);
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
    fn add_order(&self, order: &Order) {
        let mut orders = self.orders.lock().unwrap();
        orders.push(order.clone());
    }

    fn get_orders(&self) -> Result<Vec<Order>, Box<dyn std::error::Error>> {
        let orders = self.orders.lock().unwrap();
        Ok(orders.clone())
    }
}
