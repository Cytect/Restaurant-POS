#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// ============================================================
// Data Structures
// ============================================================

/// Represents a menu item in the restaurant
#[contracttype]
#[derive(Clone, Debug)]
pub struct MenuItem {
    pub id: u64,
    pub name: String,
    pub price: u64, // price in smallest unit (e.g. cents)
}

/// Represents an item within an order
#[contracttype]
#[derive(Clone, Debug)]
pub struct OrderItem {
    pub menu_item_id: u64,
    pub name: String,
    pub quantity: u32,
    pub price: u64,
}

/// Represents a customer order
#[contracttype]
#[derive(Clone, Debug)]
pub struct Order {
    pub id: u64,
    pub items: Vec<OrderItem>,
    pub total: u64,
    pub status: u32, // 0 = pending, 1 = completed
}

// ============================================================
// Storage Keys
// ============================================================

const MENU_DATA: Symbol = symbol_short!("MENU");
const ORDER_DATA: Symbol = symbol_short!("ORDERS");

// ============================================================
// Contract
// ============================================================

#[contract]
pub struct RestaurantPosContract;

#[contractimpl]
impl RestaurantPosContract {

    // ----------------------------------------------------------
    // Menu Management
    // ----------------------------------------------------------

    /// Add a new item to the restaurant menu
    pub fn add_menu_item(env: Env, name: String, price: u64) -> String {
        let mut menu: Vec<MenuItem> = env
            .storage()
            .instance()
            .get(&MENU_DATA)
            .unwrap_or(Vec::new(&env));

        let item = MenuItem {
            id: env.prng().gen::<u64>(),
            name,
            price,
        };

        menu.push_back(item);
        env.storage().instance().set(&MENU_DATA, &menu);

        String::from_str(&env, "Menu item added successfully")
    }

    /// Retrieve all menu items
    pub fn get_menu(env: Env) -> Vec<MenuItem> {
        env.storage()
            .instance()
            .get(&MENU_DATA)
            .unwrap_or(Vec::new(&env))
    }

    /// Remove a menu item by its ID
    pub fn remove_menu_item(env: Env, id: u64) -> String {
        let mut menu: Vec<MenuItem> = env
            .storage()
            .instance()
            .get(&MENU_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..menu.len() {
            if menu.get(i).unwrap().id == id {
                menu.remove(i);
                env.storage().instance().set(&MENU_DATA, &menu);
                return String::from_str(&env, "Menu item removed successfully");
            }
        }

        String::from_str(&env, "Menu item not found")
    }

    // ----------------------------------------------------------
    // Order Management
    // ----------------------------------------------------------

    /// Create a new order from menu item IDs and their quantities
    pub fn create_order(env: Env, item_ids: Vec<u64>, quantities: Vec<u32>) -> String {
        let menu: Vec<MenuItem> = env
            .storage()
            .instance()
            .get(&MENU_DATA)
            .unwrap_or(Vec::new(&env));

        let mut order_items: Vec<OrderItem> = Vec::new(&env);
        let mut total: u64 = 0;

        // Build order items by looking up each menu item
        for idx in 0..item_ids.len() {
            let target_id = item_ids.get(idx).unwrap();
            let qty = quantities.get(idx).unwrap();

            // Find the menu item
            for m in 0..menu.len() {
                let mi = menu.get(m).unwrap();
                if mi.id == target_id {
                    let line_total = mi.price * (qty as u64);
                    total += line_total;

                    order_items.push_back(OrderItem {
                        menu_item_id: mi.id,
                        name: mi.name.clone(),
                        quantity: qty,
                        price: mi.price,
                    });
                    break;
                }
            }
        }

        // Save the order
        let mut orders: Vec<Order> = env
            .storage()
            .instance()
            .get(&ORDER_DATA)
            .unwrap_or(Vec::new(&env));

        let order = Order {
            id: env.prng().gen::<u64>(),
            items: order_items,
            total,
            status: 0, // pending
        };

        orders.push_back(order);
        env.storage().instance().set(&ORDER_DATA, &orders);

        String::from_str(&env, "Order created successfully")
    }

    /// Retrieve all orders
    pub fn get_orders(env: Env) -> Vec<Order> {
        env.storage()
            .instance()
            .get(&ORDER_DATA)
            .unwrap_or(Vec::new(&env))
    }

    /// Mark an order as completed
    pub fn complete_order(env: Env, id: u64) -> String {
        let mut orders: Vec<Order> = env
            .storage()
            .instance()
            .get(&ORDER_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..orders.len() {
            let mut order = orders.get(i).unwrap();
            if order.id == id {
                order.status = 1; // completed
                orders.set(i, order);
                env.storage().instance().set(&ORDER_DATA, &orders);
                return String::from_str(&env, "Order completed successfully");
            }
        }

        String::from_str(&env, "Order not found")
    }
}

mod test;
