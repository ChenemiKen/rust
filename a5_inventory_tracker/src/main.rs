// Project: Inventory Tracker
// Define Item struct with fields (name, quantity, price).
// Implement methods to add, remove, and update items.
// Save inventory to a file (serialization with serde).
// Builds toward: Custom types, methods, file handling.
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Item{
    name: String,
    quantity: u32,
    price: f32
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Inventory{
    items: Vec<Item>
}

impl Inventory{
    fn add(&mut self, item: Item){
        self.items.push(item);
    }

    fn remove(&mut self, name: &str){
        self.items.retain(|i| i.name != name)
    }

    fn update(&mut self, idx: usize, item: Item){
        if idx < self.items.len(){
            self.items[idx] = item;
        }
    }

    fn update_quantity(&mut self, name: &str, quantity: u32){
        for item in &mut self.items{
            if item.name == name{
                item.quantity = quantity;
            }
        }
    }
    
    fn save(&self, path: &str){
        let serialized = serde_json::to_string(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }
}

fn main() {
    println!("A5 Inventory Trackers");

    let item = Item{
        name: String::from("item1"),
        quantity: 4,
        price: 4.5
    };

    let item2 = Item{
        name: String::from("item2"),
        quantity: 2,
        price: 3.0
    };

    let item3 = Item{
        name: String::from("item3"),
        quantity: 5,
        price: 7.5
    };

    let mut inventory = Inventory{
        items: Vec::new()
    };
    inventory.add(item);
    inventory.add(item2);
    println!("{:?}", inventory);

    inventory.update_quantity("item1", 10);
    println!("{:?}", inventory);

    inventory.update(1, item3);
    println!("{:?}", inventory);

    inventory.remove("item1");
    println!("{:?}", inventory);

    inventory.save("inventory.json");
}
