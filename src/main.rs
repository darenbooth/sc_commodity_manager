use std::collections::HashMap;
use std::io::{self, Write}; // Import Write for flush
use std::fs; // Import fs for file operations

// The following line is crucial for saving and loading data
use serde::{Serialize, Deserialize};

const DATA_FILE: &str = "inventory_data.json";

// Add #[derive(Serialize, Deserialize)] to make the struct compatible with Serde
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct AssetData {
    inventory: f64, 
    total_cost: f64, 
}

// Helper function to read and parse floating-point input
fn read_float_input(prompt: &str) -> Result<f64, ()> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    match input.trim().parse::<f64>() {
        Ok(num) if num >= 0.0 => Ok(num), // Ensure non-negative numbers
        _ => {
            println!("🛑 ERROR: Invalid number entered. Please enter a positive number.");
            Err(())
        }
    }
}

// Function to load the inventory from a JSON file
fn load_inventory() -> HashMap<String, AssetData> {
    match fs::read_to_string(DATA_FILE) {
        Ok(data) => {
            // Attempt to deserialize the JSON string back into the HashMap
            match serde_json::from_str(&data) {
                Ok(map) => {
                    println!("\n✅ Inventory loaded successfully from {}.", DATA_FILE);
                    map
                }
                Err(e) => {
                    eprintln!("🛑 ERROR: Could not deserialize data from {}. Starting with empty inventory. Details: {}", DATA_FILE, e);
                    HashMap::new()
                }
            }
        }
        Err(e) => {
            // This is the expected error if the file doesn't exist yet
            if e.kind() == io::ErrorKind::NotFound {
                println!("\nℹ️ {} not found. Starting with empty inventory.", DATA_FILE);
            } else {
                eprintln!("🛑 ERROR reading {}. Starting with empty inventory. Details: {}", DATA_FILE, e);
            }
            HashMap::new()
        }
    }
}

// Function to save the inventory to a JSON file
fn save_inventory(inventory_map: &HashMap<String, AssetData>) {
    // Attempt to serialize the HashMap into a JSON string
    match serde_json::to_string_pretty(inventory_map) {
        Ok(json) => {
            // Write the JSON string to the file
            match fs::write(DATA_FILE, json) {
                Ok(_) => println!("\n💾 Inventory saved successfully to {}.", DATA_FILE),
                Err(e) => eprintln!("🛑 ERROR: Could not write to {}. Details: {}", DATA_FILE, e),
            }
        }
        Err(e) => eprintln!("🛑 ERROR: Could not serialize inventory data. Details: {}", e),
    }
}


// Helper function to print the current inventory summary (no changes here)
fn print_summary(inventory_map: &HashMap<String, AssetData>) {
    println!("\n--- CURRENT INVENTORY SUMMARY ---");
    if inventory_map.is_empty() {
        println!("Inventory is currently empty.");
        println!("---------------------------------");
        return;
    }
    
    // Sort keys alphabetically for a cleaner display
    let mut keys: Vec<&String> = inventory_map.keys().collect();
    keys.sort();

    for key in keys {
        let data = inventory_map.get(key).unwrap();
        let avg_cost = if data.inventory > 0.0 {
            data.total_cost / data.inventory
        } else {
            0.0
        };
        
        println!("  - **{}**", key);
        println!("    > SCU: {:.2}", data.inventory);
        println!("    > Cost: {:.2} Credits", data.total_cost);
        println!("    > Avg. Cost/Unit: {:.2} Credits", avg_cost);
    }
    println!("---------------------------------");
}

// Function to pause the execution and wait for user input
fn wait_for_exit() {
    println!("\nPress ENTER or RETURN to close the window...");
    // Flush stdout to ensure the prompt is displayed immediately
    let _ = io::stdout().flush();
    // Read a line from stdin, effectively pausing the program until Enter is pressed
    let mut exit_input = String::new();
    let _ = io::stdin().read_line(&mut exit_input);
}

fn main() {
    // 1. Load inventory at startup
    let mut inventory_map = load_inventory();
    let mut session_profit_loss: f64 = 0.0; 

    println!("⭐ Welcome to the SC Commodity Manager (CLI) ⭐");
    print_summary(&inventory_map);

    // Main transaction loop
    loop {
        // --- Get Transaction Type ---
        println!("\n--- New Transaction ---");
        println!("Type 'B' to **BUY**, 'S' to **SELL**, or 'Q' to **QUIT**:");
        
        // Ensure prompt is displayed before waiting for input
        let _ = io::stdout().flush(); 

        let mut transaction_type = String::new();
        io::stdin().read_line(&mut transaction_type).expect("Failed to read line");
        let transaction_type = transaction_type.trim().to_uppercase();

        if transaction_type == "Q" {
            println!("\nApplication quitting. Final Summary:");
            break; 
        }

        if transaction_type != "B" && transaction_type != "S" {
            println!("Invalid input. Please enter 'B', 'S', or 'Q'.");
            continue;
        }

        // --- Get Asset Name ---
        println!("Enter the name of the asset (e.g., Astatine):");
        let mut asset_name = String::new();
        io::stdin().read_line(&mut asset_name).expect("Failed to read line");
        let asset_name = asset_name.trim().to_string();
        
        if asset_name.is_empty() {
            println!("Asset name cannot be empty.");
            continue;
        }

        // --- Get Quantity (SCU) ---
        let quantity = match read_float_input("Enter the quantity (in SCU) for this transaction:") {
            Ok(q) => q,
            Err(_) => continue,
        };

        // --- Get Transaction Amount ---
        let amount_prompt = match transaction_type.as_str() {
            "B" => "Enter the **TOTAL PURCHASE COST** (in aUEC):",
            "S" => "Enter the **TOTAL SALE REVENUE** (in aUEC):",
            _ => "Enter the total transaction amount (in aUEC):",
        };
        let amount = match read_float_input(amount_prompt) {
            Ok(a) => a,
            Err(_) => continue,
        };

        // --- Get Transportation Fees ---
        let transport_fees = match read_float_input("Enter the **TOTAL TRANSPORT FEES** associated with this transaction (0 if none):") {
            Ok(f) => f,
            Err(_) => continue,
        };


        // --- Process Transaction (No changes to logic) ---
        let asset_data = inventory_map.entry(asset_name.clone()).or_insert(AssetData {
            inventory: 0.0,
            total_cost: 0.0,
        });

        match transaction_type.as_str() {
            "B" => {
                asset_data.inventory += quantity;
                asset_data.total_cost += amount + transport_fees; 
                println!("\n✅ Buy transaction processed for {}. Inventory increased (Fees included in cost).", asset_name);
            }
            "S" => {
                if quantity > asset_data.inventory {
                    println!("\n🛑 ERROR: Cannot sell {:.2} SCU of {}, only {:.2} SCU is in inventory.", quantity, asset_name, asset_data.inventory);
                    continue;
                }

                // Calculate Net Profit/Loss
                let avg_cost_per_unit = if asset_data.inventory > 0.0 {
                    asset_data.total_cost / asset_data.inventory
                } else {
                    0.0
                };
                
                let cost_of_goods_sold = quantity * avg_cost_per_unit;
                let net_profit_loss = amount - cost_of_goods_sold - transport_fees;
                session_profit_loss += net_profit_loss; 
                
                // Update Inventory and Total Cost
                asset_data.inventory -= quantity;
                asset_data.total_cost -= cost_of_goods_sold;
                
                // Output Result
                if net_profit_loss >= 0.0 {
                    println!("\n🎉 SALE SUCCESSFUL! Net Profit on {} sale: {:.2} aUEC", asset_name, net_profit_loss);
                } else {
                    println!("\n💔 SALE SUCCESSFUL! Net Loss on {} sale: {:.2} aUEC", asset_name, net_profit_loss);
                }
                println!("    (Total Revenue: {:.2}, COGS: {:.2}, Fees: {:.2})", amount, cost_of_goods_sold, transport_fees);

                // Cleanup
                if asset_data.inventory <= 0.0 {
                    inventory_map.remove(&asset_name); 
                    println!("Inventory cleared for {}.", asset_name);
                } else {
                    if asset_data.total_cost < 0.0 {
                        asset_data.total_cost = 0.0;
                    }
                }
            }
            _ => { /* Already handled */ }
        }

        // --- Summary after each transaction ---
        print_summary(&inventory_map);
    } // End of loop

    // 2. Save inventory before exiting
    save_inventory(&inventory_map);
    
    // Final session summary
    println!("\n=== FINAL SESSION SUMMARY ===");
    if session_profit_loss >= 0.0 {
        println!("💰 **TOTAL SESSION NET PROFIT:** {:.2} aUEC", session_profit_loss);
    } else {
        println!("🔻 **TOTAL SESSION NET LOSS:** {:.2} aUEC", session_profit_loss);
    }
    println!("=============================");
    print_summary(&inventory_map);
    
    // Pause before closing the window
    wait_for_exit();
} // End of main function

