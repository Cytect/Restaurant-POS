#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, Vec};

#[test]
fn test_add_and_get_menu() {
    let env = Env::default();
    let contract_id = env.register(RestaurantPosContract, ());
    let client = RestaurantPosContractClient::new(&env, &contract_id);

    // Add menu items
    client.add_menu_item(&String::from_str(&env, "Nasi Goreng"), &15000);
    client.add_menu_item(&String::from_str(&env, "Mie Ayam"), &12000);

    // Get menu
    let menu = client.get_menu();
    assert_eq!(menu.len(), 2);
}

#[test]
fn test_remove_menu_item() {
    let env = Env::default();
    let contract_id = env.register(RestaurantPosContract, ());
    let client = RestaurantPosContractClient::new(&env, &contract_id);

    // Add item
    client.add_menu_item(&String::from_str(&env, "Sate Ayam"), &20000);

    let menu = client.get_menu();
    assert_eq!(menu.len(), 1);

    let item_id = menu.get(0).unwrap().id;

    // Remove it
    client.remove_menu_item(&item_id);

    let menu_after = client.get_menu();
    assert_eq!(menu_after.len(), 0);
}

#[test]
fn test_create_and_get_order() {
    let env = Env::default();
    let contract_id = env.register(RestaurantPosContract, ());
    let client = RestaurantPosContractClient::new(&env, &contract_id);

    // Add menu items first
    client.add_menu_item(&String::from_str(&env, "Nasi Goreng"), &15000);
    client.add_menu_item(&String::from_str(&env, "Es Teh"), &5000);

    let menu = client.get_menu();
    let id1 = menu.get(0).unwrap().id;
    let id2 = menu.get(1).unwrap().id;

    // Create order: 2x Nasi Goreng + 1x Es Teh
    let mut item_ids: Vec<u64> = Vec::new(&env);
    item_ids.push_back(id1);
    item_ids.push_back(id2);

    let mut quantities: Vec<u32> = Vec::new(&env);
    quantities.push_back(2);
    quantities.push_back(1);

    client.create_order(&item_ids, &quantities);

    let orders = client.get_orders();
    assert_eq!(orders.len(), 1);

    let order = orders.get(0).unwrap();
    // 2 * 15000 + 1 * 5000 = 35000
    assert_eq!(order.total, 35000);
    assert_eq!(order.status, 0); // pending
}

#[test]
fn test_complete_order() {
    let env = Env::default();
    let contract_id = env.register(RestaurantPosContract, ());
    let client = RestaurantPosContractClient::new(&env, &contract_id);

    // Add a menu item
    client.add_menu_item(&String::from_str(&env, "Bakso"), &10000);

    let menu = client.get_menu();
    let id = menu.get(0).unwrap().id;

    // Create order
    let mut item_ids: Vec<u64> = Vec::new(&env);
    item_ids.push_back(id);
    let mut quantities: Vec<u32> = Vec::new(&env);
    quantities.push_back(1);

    client.create_order(&item_ids, &quantities);

    let orders = client.get_orders();
    let order_id = orders.get(0).unwrap().id;

    // Complete the order
    client.complete_order(&order_id);

    let updated_orders = client.get_orders();
    assert_eq!(updated_orders.get(0).unwrap().status, 1); // completed
}
