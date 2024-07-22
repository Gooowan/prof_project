mod models;
mod operations;

use operations::{read_sneakers_from_file, read_cart_from_file};
use std::io::{stdout, Write};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor,
};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let (sneakers, carts) = read_data().unwrap();
    let (mut current_page, items_per_page, total_pages) = initialize_pagination(&sneakers);

    main_loop(sneakers, carts, &mut current_page, items_per_page, total_pages);
}

fn read_data() -> Result<(Vec<models::Sneaker>, Vec<models::Cart>), Box<dyn std::error::Error>> {
    let data_dir = "../data/";

    let sneakers = read_sneakers_from_file(&format!("{}catalogue.tsv", data_dir))?;
    let carts = read_cart_from_file(&format!("{}cart.tsv", data_dir))?;
    
    Ok((sneakers, carts))
}

fn initialize_pagination(sneakers: &Vec<models::Sneaker>) -> (usize, usize, usize) {
    let current_page = 0;
    let items_per_page = 9;
    let total_pages = (sneakers.len() + items_per_page - 1) / items_per_page;
    
    (current_page, items_per_page, total_pages)
}

fn main_loop(
    sneakers: Vec<models::Sneaker>,
    carts: Vec<models::Cart>,
    current_page: &mut usize,
    items_per_page: usize,
    total_pages: usize
) {
    let mut stdout = stdout();
    let mut rl = Editor::<()>::new();

    loop {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

        println!("Controls: 1-9 to select an item, A for previous page, D for next page");
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
                    _ => {
                        if let Ok(selected_index) = line.parse::<usize>() {
                            let selected_index = selected_index - 1 + *current_page * items_per_page;
                            if selected_index < sneakers.len() {
                                // Handle valid sneaker selection
                                println!("Selected item: {}", sneakers[selected_index].name);
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
