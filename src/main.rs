mod models;
mod operations;

use models::{Cart, Sneaker};
use operations::{read_sneakers_from_file};
use std::io::{stdout, Write};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor,
};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs::OpenOptions;
use std::path::Path;

fn main() {
    let (sneakers, mut carts) = read_data().unwrap();
    let (mut current_page, items_per_page, total_pages) = initialize_pagination(&sneakers);

    main_loop(sneakers, &mut carts, &mut current_page, items_per_page, total_pages);
}

fn read_data() -> Result<(Vec<Sneaker>, Vec<Cart>), Box<dyn std::error::Error>> {
    let data_dir = "../data/";

    let sneakers = read_sneakers_from_file(&format!("{}catalogue.tsv", data_dir))?;
    let carts = Vec::new(); // Initialize an empty cart
    
    Ok((sneakers, carts))
}

fn initialize_pagination(sneakers: &Vec<Sneaker>) -> (usize, usize, usize) {
    let current_page = 0;
    let items_per_page = 9;
    let total_pages = (sneakers.len() + items_per_page - 1) / items_per_page;
    
    (current_page, items_per_page, total_pages)
}

fn main_loop(
    sneakers: Vec<Sneaker>,
    carts: &mut Vec<Cart>,
    current_page: &mut usize,
    items_per_page: usize,
    total_pages: usize
) {
    let mut stdout = stdout();
    let mut rl = Editor::<()>::new();

    loop {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

        println!("Controls: 1-9 to select an item, A for previous page, D for next page, C to view cart");
        println!("Page {}/{}", *current_page + 1, total_pages);

        let start_index = *current_page * items_per_page;
        let end_index = std::cmp::min(start_index + items_per_page, sneakers.len());
        for (i, sneaker) in sneakers[start_index..end_index].iter().enumerate() {
            println!("{}: {} - ${}", i + 1, sneaker.name, sneaker.price);
        }

        stdout.flush().unwrap();

        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                match line.trim().to_uppercase().as_str() {
                    "A" => {
                        if *current_page > 0 {
                            *current_page -= 1;
                        } else {
                            *current_page = total_pages - 1;
                        }
                    }
                    "D" => {
                        if *current_page < total_pages - 1 {
                            *current_page += 1;
                        } else {
                            *current_page = 0;
                        }
                    }
                    "C" => {
                        view_cart(&sneakers, carts);
                    }
                    _ => {
                        if let Ok(selected_index) = line.parse::<usize>() {
                            let selected_index = selected_index - 1 + *current_page * items_per_page;
                            if selected_index < sneakers.len() {
                                // Call function to display detailed sneaker info
                                display_sneaker_info(&sneakers[selected_index], carts);
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn display_sneaker_info(sneaker: &Sneaker, carts: &mut Vec<Cart>) {
    let mut stdout = stdout();
    loop {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

        println!("\nDetailed Information:");
        println!("ID: {}", sneaker.sneaker_id);
        println!("Name: {}", sneaker.name);
        println!("Brand: {}", sneaker.brand);
        println!("Price: ${}", sneaker.price);
        println!("Description: {}", sneaker.description);
        println!("Images: {}", sneaker.images);
        println!("\nPress B to add to cart, or any other key to return to the list...");

        stdout.flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_uppercase().as_str() {
            "B" => {
                add_to_cart(sneaker, carts);
                println!("Added to cart!");
            }
            _ => break,
        }

        println!("\nPress Enter to return to the list...");
        std::io::stdin().read_line(&mut input).unwrap();
        break;
    }
}

fn add_to_cart(sneaker: &Sneaker, carts: &mut Vec<Cart>) {
    let cart_item = Cart {
        sneaker_id: sneaker.sneaker_id,
        total_price: sneaker.price,
    };
    carts.push(cart_item);
}

fn view_cart(sneakers: &Vec<Sneaker>, carts: &mut Vec<Cart>) {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

    if carts.is_empty() {
        println!("Your cart is empty.");
    } else {
        loop {
            println!("Your cart items:");
            for (i, cart) in carts.iter().enumerate() {
                if let Some(sneaker) = sneakers.iter().find(|&s| s.sneaker_id == cart.sneaker_id) {
                    println!("{}: Sneaker: {}, Price: ${}", i + 1, sneaker.name, cart.total_price);
                } else {
                    println!("{}: Sneaker ID: {}, Price: ${}", i + 1, cart.sneaker_id, cart.total_price);
                }
            }

            let total_price: f64 = carts.iter().map(|cart| cart.total_price).sum();
            println!("\nTotal Price: ${}", total_price);
            println!("Press F to finish the order, R n to remove an item (where n is the item number), or any other key to return to the list...");
            
            stdout.flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_uppercase();

            if input.starts_with("F") {
                let order_id = generate_order_id();
                match finished_order(carts, order_id) {
                    Ok(_) => {
                        println!("Order finished! Total amount: ${}", total_price);
                        carts.clear();
                    }
                    Err(e) => {
                        println!("Failed to finish order: {}", e);
                    }
                }
                break;
            } else if input.starts_with("R") {
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() == 2 {
                    if let Ok(index) = parts[1].parse::<usize>() {
                        if index > 0 && index <= carts.len() {
                            carts.remove(index - 1);
                            println!("Item removed from the cart.");
                        } else {
                            println!("Invalid item number.");
                        }
                    } else {
                        println!("Invalid command format.");
                    }
                } else {
                    println!("Invalid command format.");
                }
            } else {
                break;
            }

            println!("\nPress Enter to return to the list...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }
    }

    println!("\nPress Enter to return to the list...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}


fn finished_order(carts: &Vec<Cart>, order_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = "../data/";
    let orders_file = format!("{}orders.tsv", data_dir);
    let orders_path = Path::new(&orders_file);

    let sneaker_ids: Vec<String> = carts.iter().map(|cart| cart.sneaker_id.to_string()).collect();
    let total_value: f64 = carts.iter().map(|cart| cart.total_price).sum();

    let mut file = OpenOptions::new().append(true).create(true).open(orders_path)?;

    writeln!(file, "{}, {}, {}", order_id, sneaker_ids.join(","), total_value)?;

    Ok(())
}

fn generate_order_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs() as u32
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{BufRead, BufReader};

    fn create_test_sneakers() -> Vec<Sneaker> {
        vec![
            Sneaker {
                sneaker_id: 1,
                name: "Sneaker One".to_string(),
                brand: "Brand A".to_string(),
                price: 100.0,
                description: "Description One".to_string(),
                images: "Image One".to_string(),
            },
            Sneaker {
                sneaker_id: 2,
                name: "Sneaker Two".to_string(),
                brand: "Brand B".to_string(),
                price: 200.0,
                description: "Description Two".to_string(),
                images: "Image Two".to_string(),
            },
        ]
    }

    #[test]
    fn test_add_to_cart() {
        let sneakers = create_test_sneakers();
        let mut carts = Vec::new();
        add_to_cart(&sneakers[0], &mut carts);
        assert_eq!(carts.len(), 1);
        assert_eq!(carts[0].sneaker_id, 1);
        assert_eq!(carts[0].total_price, 100.0);
    }

    #[test]
    fn test_remove_from_cart() {
        let sneakers = create_test_sneakers();
        let mut carts = Vec::new();
        add_to_cart(&sneakers[0], &mut carts);
        add_to_cart(&sneakers[1], &mut carts);
        assert_eq!(carts.len(), 2);
        carts.remove(0);
        assert_eq!(carts.len(), 1);
        assert_eq!(carts[0].sneaker_id, 2);
    }
}
