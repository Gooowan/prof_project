use crate::models::{Sneaker, User, Cart, Order};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_sneakers_from_file(file_path: &str) -> Result<Vec<Sneaker>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut sneakers = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        let sneaker = Sneaker {
            sneaker_id: fields[0].parse()?,
            name: fields[1].to_string(),
            brand: fields[2].to_string(),
            price: fields[3].parse()?,
            description: fields[4].to_string(),
            images: fields[5].to_string(),
        };
        sneakers.push(sneaker);
    }
    Ok(sneakers)
}

pub fn read_users_from_file(file_path: &str) -> Result<Vec<User>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut users = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        let user = User {
            user_id: fields[0].parse()?,
            username: fields[1].to_string(),
            password: fields[2].to_string(),
            email: fields[3].to_string(),
            created_at: fields[4].to_string(),
        };
        users.push(user);
    }
    Ok(users)
}

pub fn read_cart_from_file(file_path: &str) -> Result<Vec<Cart>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut carts = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        let cart = Cart {
            cart_id: fields[0].parse()?,
            user_id: fields[1].parse()?,
            sneaker_id: fields[2].parse()?,
            quantity: fields[3].parse()?,
        };
        carts.push(cart);
    }
    Ok(carts)
}

pub fn read_orders_from_file(file_path: &str) -> Result<Vec<Order>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut orders = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        let order = Order {
            order_id: fields[0].parse()?,
            user_id: fields[1].parse()?,
            total_price: fields[2].parse()?,
            order_date: fields[3].to_string(),
            items: fields[4].to_string(),
        };
        orders.push(order);
    }
    Ok(orders)
}
