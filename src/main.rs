mod inventory;
mod orders;

fn main() {
    println!("The manager of our invertory is: {}", inventory::MANAGER);
    println!("The manager of our orders is: {}", orders::MANAGER);
}