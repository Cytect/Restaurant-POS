# Restaurant POS — Stellar Soroban Smart Contract

A decentralized Restaurant Point-of-Sale (POS) system built on the Stellar blockchain using Soroban smart contracts. This application allows restaurant owners to manage their menu items and track customer orders directly on-chain.

## Features

**Menu Management**
- **Add Menu Item** ([add_menu_item](cci:1://file:///c:/Users/Lenovo/Desktop/Main/stellar%20blockchain/Restaurant-POS/contracts/restaurant_pos/src/lib.rs:57:4-75:5)) — Register new dishes with name and price
- **View Menu** ([get_menu](cci:1://file:///c:/Users/Lenovo/Desktop/Main/stellar%20blockchain/Restaurant-POS/contracts/restaurant_pos/src/lib.rs:77:4-83:5)) — Retrieve all available menu items
- **Remove Menu Item** ([remove_menu_item](cci:1://file:///c:/Users/Lenovo/Desktop/Main/stellar%20blockchain/Restaurant-POS/contracts/restaurant_pos/src/lib.rs:85:4-102:5)) — Delete items from the menu by ID

**Order Management**
- **Create Order** ([create_order](cci:1://file:///c:/Users/Lenovo/Desktop/Main/stellar%20blockchain/Restaurant-POS/contracts/restaurant_pos/src/lib.rs:108:4-160:5)) — Place orders with multiple items, auto-calculates totals
- **View Orders** ([get_orders](cci:1://file:///c:/Users/Lenovo/Desktop/Main/stellar%20blockchain/Restaurant-POS/contracts/restaurant_pos/src/lib.rs:162:4-168:5)) — Retrieve all orders and their statuses
- **Complete Order** ([complete_order](cci:1://file:///c:/Users/Lenovo/Desktop/Main/stellar%20blockchain/Restaurant-POS/contracts/restaurant_pos/src/lib.rs:170:4-189:5)) — Mark an order as completed

## Contract Details
- **Network**: Stellar Testnet
- **Contract ID**: _(Paste your deployed Contract ID here!)_

## Interacting on Testnet
1. Add a menu item: [add_menu_item](cci:1://file:///c:/Users/Lenovo/Desktop/Main/stellar%20blockchain/Restaurant-POS/contracts/restaurant_pos/src/lib.rs:57:4-75:5) with `name` = "Nasi Goreng", `price` = 15000
2. Check the menu: [get_menu](cci:1://file:///c:/Users/Lenovo/Desktop/Main/stellar%20blockchain/Restaurant-POS/contracts/restaurant_pos/src/lib.rs:77:4-83:5) (Copy the auto-generated ID)
3. Create an order: [create_order](cci:1://file:///c:/Users/Lenovo/Desktop/Main/stellar%20blockchain/Restaurant-POS/contracts/restaurant_pos/src/lib.rs:108:4-160:5) with `item_ids` = [your_id], `quantities` = [2]
4. Check orders: [get_orders](cci:1://file:///c:/Users/Lenovo/Desktop/Main/stellar%20blockchain/Restaurant-POS/contracts/restaurant_pos/src/lib.rs:162:4-168:5)
5. Complete: [complete_order](cci:1://file:///c:/Users/Lenovo/Desktop/Main/stellar%20blockchain/Restaurant-POS/contracts/restaurant_pos/src/lib.rs:170:4-189:5) with the order `id`
