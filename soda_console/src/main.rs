use std::io::{self, Write};
use std::sync::Arc;

use memory_repository::InMemorySodaMachineRepository;
use soda_core::application::customer_service::CustomerService;
use soda_core::application::operator_service::OperatorService;
use soda_core::ports::driving::customer_port::CustomerPort;
use soda_core::ports::driving::operator_port::OperatorPort;
use soda_core::domain::value_objects::soda::{Soda,SodaFlavor,SodaSize};
use soda_core::domain::value_objects::money::Money;

async fn seed(operator_service: &OperatorService) {
    // Add a soda machine with ID 1 and max 5 slots
    let machine_id = 1u32;
    let max_slots = 5u32;
    if let Err(e) = operator_service.create_new_machine(machine_id, max_slots).await {
        println!("Failed to add machine: {:?}", e);
    }

    // Add a slot with ID 1 and capacity 10
    let slot_id = 1u32;
    let slot_capacity = 10u32;
    if let Err(e) = operator_service.configure_slot(machine_id, slot_id, slot_capacity, Soda::new(
        "Cola".to_string(),
        SodaFlavor::Cola,
        SodaSize::Medium,
        Money::from_cents(125),
        false,
        false,
    ).unwrap()).await {
        println!("Failed to configure slot: {:?}", e);
    }

    // Refill the slot with 3 sodas
    if let Err(e) = operator_service.refill_slot(machine_id, slot_id, 3).await {
        println!("Failed to refill slot: {:?}", e);
    }
}

#[tokio::main]
async fn main() {
    let repo = Arc::new(InMemorySodaMachineRepository::new());
    let customer_service = Arc::new(CustomerService::new(repo.clone()));
    let operator_service = Arc::new(OperatorService::new(repo.clone()));

    seed(&operator_service).await;
    
    loop {
        println!("\nWelcome to Soda Console!");
        println!("1. Soda Consumer");
        println!("2. Soda Operator");
        println!("3. Exit");
        print!("Select your role: ");
        io::stdout().flush().unwrap();

        let mut role = String::new();
        io::stdin().read_line(&mut role).unwrap();
        let role = role.trim();

        match role {
            "1" => soda_consumer_menu(customer_service.clone()).await,
            "2" => soda_operator_menu(operator_service.clone()).await,
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}

async fn soda_consumer_menu(customer_service: Arc<CustomerService>) {
    println!("\n--- Soda Consumer ---");
    println!("1. Available Sodas");
    println!("2. Insert Money");
    println!("3. Buy Soda");
    println!("4. Request Money Back");
    print!("Select an option: ");
    io::stdout().flush().unwrap();

    let mut op = String::new();
    io::stdin().read_line(&mut op).unwrap();
    let op = op.trim();

    match op {
        "1" => {
            // List available sodas in all machines
            let id = prompt("Enter Soda Machine ID: ");
            let id: u32 = id.parse().unwrap_or(0);

            match customer_service.list_available_sodas(id).await {
                Ok(sodas) => {
                    if sodas.is_empty() {
                        println!("No sodas available in this machine.");
                    } else {
                        println!("Available Sodas:");
                        for soda in sodas {
                            println!(
                                "Slot {}: {} - ${}",
                                soda.slot_id,
                                soda.soda_name,
                                soda.price
                            );
                        }
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        "2" => {
            // Insert money
            let id = prompt("Enter Soda Machine ID: ");
            let id: u32 = id.parse().unwrap_or(0);

            let amount = prompt("Enter amount to insert (e.g., 2.50): ");
            let amount: f64 = amount.parse().unwrap_or(0.0);

            // Convert to cents to avoid floating point issues
            let dollars = amount.trunc() as i64;
            let cents = ((amount.fract() * 100.0).round()) as u8;

            match soda_core::domain::value_objects::money::Money::from_dollars_cents(dollars, cents) {
                Ok(money) => {
                    match customer_service.insert_money(id, money).await {
                        Ok(_) => println!("Money inserted successfully."),
                        Err(e) => println!("Error: {}", e),
                    }
                }
                Err(e) => println!("Invalid amount: {}", e),
            }
        }
        "3" => {
            // Buy soda
            let id = prompt("Enter Soda Machine ID: ");
            let id: u32 = id.parse().unwrap_or(0);

            let slot_id = prompt("Enter Slot ID to buy: ");
            let slot_id: u32 = slot_id.parse().unwrap_or(0);

            match customer_service.buy_soda(id, slot_id).await {
                Ok(_) => println!("Enjoy your soda!"),
                Err(e) => println!("Error: {}", e),
            }
        }
        "4" => {
            // Request money back
            let id = prompt("Enter Soda Machine ID: ");
            let id: u32 = id.parse().unwrap_or(0);

            match customer_service.request_money_back(id).await {
                Ok(money) => {
                    println!("Returned: ${:.2}", money.as_decimal());
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        _ => {
            println!("Invalid option. Please try again.");
        }
    }
}

async fn soda_operator_menu(operator_service: Arc<OperatorService>) {
    println!("\n--- Soda Operator ---");
    println!("1. Create Soda Machine");
    println!("2. View Soda Machine");
    println!("3. Add Slot to Soda Machine");
    println!("4. Refill Slot in Soda Machine");
    print!("Select an option: ");
    io::stdout().flush().unwrap();

    let mut op = String::new();
    io::stdin().read_line(&mut op).unwrap();
    let op = op.trim();

    match op {
        "1" => {
            let id = prompt("Enter new Soda Machine ID: ");
            let id: u32 = id.parse().unwrap_or(0);

            match operator_service.create_new_machine(id, 10).await {
                Ok(_) => println!("Soda Machine created."),
                Err(e) => println!("Error: {}", e),
            }
        }
        "2" => {
            let id = prompt("Enter Soda Machine ID to view: ");
            let id: u32 = id.parse().unwrap_or(0);

            match operator_service.get_machine_status(id).await {
                Ok(machine) => println!("Soda Machine: {}", machine),
                Err(e) => println!("Error: {}", e),
            }
        }
        "3" => {
            let id = prompt("Enter Soda Machine ID to add slot: ");
            let id: u32 = id.parse().unwrap_or(0);

            let slot_id = prompt("Enter Slot ID: ");
            let slot_id: u32 = slot_id.parse().unwrap_or(0);

            let capacity = prompt("Enter Slot Capacity: ");
            let capacity: u32 = capacity.parse().unwrap_or(0);

            let name = prompt("Enter Soda Name: ");
            let flavor = prompt("Enter Soda Flavor (e.g., Cola, Orange, Lemon): ");
            let flavor = match flavor.to_lowercase().as_str() {
                "cola" => SodaFlavor::Cola,
                "orange" => SodaFlavor::Orange,
                "lemonlime" | "lemon-lime" | "lemon" | "lime" => SodaFlavor::LemonLime,
                "rootbeer" | "root-beer" | "root beer" => SodaFlavor::RootBeer,
                "grape" => SodaFlavor::Grape,
                "cherry" => SodaFlavor::Cherry,
                "vanilla" => SodaFlavor::Vanilla,
                "strawberry" => SodaFlavor::Strawberry,
                "peach" => SodaFlavor::Peach,
                "watermelon" => SodaFlavor::Watermelon,
                _ => {
                    println!("Unknown flavor, defaulting to Cola.");
                    SodaFlavor::Cola
                }
            };
            let size = prompt("Enter Soda Size (Small, Medium, Large, XLarge): ");
            let size = match size.to_lowercase().as_str() {
                "small" => SodaSize::Small,
                "medium" => SodaSize::Medium,
                "large" => SodaSize::Large,
                "xlarge" | "x-large" | "extra large" | "xl" => SodaSize::XLarge,
                _ => {
                    println!("Unknown size, defaulting to Medium.");
                    SodaSize::Medium
                }
            };
            let price = prompt("Enter Soda Price (cents): ");
            let price = match price.parse::<i64>() {
                Ok(val) => {
                    Money::from_cents(val) 
                }
                Err(_) => {
                    println!("Invalid price format, defaulting to $1.00.");
                    Money::from_cents(100)
                }
            };
            let is_diet = prompt("Is the soda diet? (y/n): ");
            let is_diet = matches!(is_diet.to_lowercase().as_str(), "y" | "yes");
            let is_caffeinated = prompt("Is the soda caffeinated? (y/n): ");
            let is_caffeinated = matches!(is_caffeinated.to_lowercase().as_str(), "y" | "yes");

            let soda = match Soda::new(
                name,
                flavor,
                size,
                price,
                is_diet,
                is_caffeinated,
            ) {
                Ok(soda) => soda,
                Err(e) => {
                    println!("Error creating soda: {:?}", e);
                    panic!("Failed to create soda");
                }
            };

            match operator_service.configure_slot(id, slot_id, capacity, soda).await {
                Ok(_) => println!("Slot added."),
                Err(e) => println!("Error: {}", e),
            }
        }
        "4" => {
            let id = prompt("Enter Soda Machine ID: ");
            let id = id.parse::<u32>().unwrap_or(1);
            let slot_id = prompt("Enter Slot ID to refill: ");
            let slot_id = slot_id.parse::<u32>().unwrap_or(1);
            let quantity = prompt("Enter quantity to add: ");
            let quantity = quantity.parse::<u32>().unwrap_or(0);

            match operator_service.refill_slot(id, slot_id, quantity).await {
                Ok(_) => println!("Slot refilled."),
                Err(e) => println!("Error: {}", e),
            }
        }
        _ => println!("Invalid option."),
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
