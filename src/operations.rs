use crate::models::{Sneaker, Cart, Order};
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