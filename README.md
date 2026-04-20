# Restaurant POS — Stellar Soroban Smart Contract

A decentralized Restaurant Point-of-Sale (POS) system built on the Stellar blockchain using Soroban smart contracts. This application allows restaurant owners to manage their menu items and track customer orders directly on-chain.

## Project Description

Restaurant POS is a blockchain-based solution that brings transparency and immutability to restaurant operations. Menu items and orders are stored on the Stellar blockchain, ensuring tamper-proof records and eliminating the need for centralized database management.

## Features

### Menu Management
- **Add Menu Item** — Register new dishes with name and price
- **View Menu** — Retrieve all available menu items
- **Remove Menu Item** — Delete items from the menu by ID

### Order Management
- **Create Order** — Place orders with multiple menu items and quantities, with automatic total calculation
- **View Orders** — Retrieve all orders and their statuses
- **Complete Order** — Mark an order as completed (fulfilled)

## Smart Contract Functions

| Function | Parameters | Description |
|---|---|---|
| `add_menu_item` | `name: String, price: u64` | Add a new item to the menu |
| `get_menu` | — | Get all menu items |
| `remove_menu_item` | `id: u64` | Remove a menu item |
| `create_order` | `item_ids: Vec<u64>, quantities: Vec<u32>` | Create a new order |
| `get_orders` | — | Get all orders |
| `complete_order` | `id: u64` | Mark order as completed |

## Contract Details

- **Network**: Stellar Testnet
- **Contract ID**: _(to be updated after deployment)_

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network

## Getting Started

### Prerequisites
- Rust & Cargo installed
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Soroban CLI: `cargo install soroban-cli`

### Build
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Test
```bash
cargo test
```

### Deploy to Testnet
```bash
# Generate wallet identity
stellar keys generate --global deployer --network testnet

# Deploy the contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/restaurant_pos.wasm \
  --source deployer \
  --network testnet
```

### Interact with the Contract
```bash
# Add a menu item
stellar contract invoke --id <CONTRACT_ID> --source deployer --network testnet \
  -- add_menu_item --name "Nasi Goreng" --price 15000

# Get menu
stellar contract invoke --id <CONTRACT_ID> --source deployer --network testnet \
  -- get_menu

# Create an order
stellar contract invoke --id <CONTRACT_ID> --source deployer --network testnet \
  -- create_order --item_ids '[123456]' --quantities '[2]'

# Get orders
stellar contract invoke --id <CONTRACT_ID> --source deployer --network testnet \
  -- get_orders

# Complete an order
stellar contract invoke --id <CONTRACT_ID> --source deployer --network testnet \
  -- complete_order --id 789012
```

## Project Structure
```
Restaurant-POS/
├── Cargo.toml                  # Workspace config
├── contracts/
│   └── restaurant_pos/
│       ├── Cargo.toml          # Contract dependencies
│       └── src/
│           ├── lib.rs          # Smart contract logic
│           └── test.rs         # Unit tests
└── README.md
```

---

Restaurant POS — Decentralized Restaurant Management on Stellar
