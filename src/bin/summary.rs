use project_structure::{FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );
}