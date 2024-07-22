#[derive(Debug, Clone)]
pub struct Sneaker {
    pub sneaker_id: u32,
    pub name: String,
    pub brand: String,
    pub price: f64,
    pub description: String,
    pub images: String,
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct Cart {
    pub cart_id: u32,
    pub user_id: u32,
    pub sneaker_id: u32,
    pub quantity: u32,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub order_id: u32,
    pub user_id: u32,
    pub total_price: f64,
    pub order_date: String,
    pub items: String,
}
