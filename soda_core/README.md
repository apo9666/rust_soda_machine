# Soda Core Domain Library

The core domain library for the Rust Soda Machine project. This library contains all the business logic, domain objects, and rules that define how a soda machine operates.

## 🏗️ Domain Model

### Value Objects
Immutable objects that represent concepts in the domain:

- **`Money`**: Precision-safe monetary calculations with overflow protection
- **`Soda`**: Product definitions with flavors, sizes, and properties

### Entities
Objects with identity and lifecycle:

- **`Slot`**: Inventory management with unique identification

### Aggregates
Consistency boundaries that orchestrate domain operations:

- **`SodaMachine`**: Main business orchestrator managing slots and customer operations

## 🚀 Quick Start

```rust
use soda_core::domain::aggregates::soda_machine::{SodaMachine, SodaMachineId};
use soda_core::domain::entities::slot::SlotId;
use soda_core::domain::value_objects::soda::{Soda, SodaFlavor, SodaSize};
use soda_core::domain::value_objects::money::Money;

// Create and configure a soda machine
let mut machine = SodaMachine::new(SodaMachineId::new(1), 10).unwrap();
machine.add_slot(SlotId::new(1), 20).unwrap();

let coke = Soda::new(
    "Coca-Cola".to_string(),
    SodaFlavor::Cola,
    SodaSize::Medium,
    Money::from_dollars_cents(1, 50).unwrap(),
    false,
    true,
).unwrap();

machine.configure_slot(SlotId::new(1), coke).unwrap();
machine.refill_slot(SlotId::new(1), 10).unwrap();

// Customer operations
machine.insert_money(Money::from_dollars_cents(2, 00).unwrap()).unwrap();
let event = machine.dispense_soda(SlotId::new(1)).unwrap();
```

## 📚 API Reference

### Money Value Object
```rust
// Creation
let money = Money::from_dollars_cents(5, 25).unwrap(); // $5.25
let money = Money::from_decimal(5.25).unwrap();        // $5.25
let money = Money::from_cents(525);                    // $5.25

// Operations
let sum = (money1 + money2).unwrap();
let diff = (money1 - money2).unwrap();
let product = (money * 2).unwrap();
let quotient = money / 2;

// Properties
let dollars = money.dollars();        // 5
let cents = money.cents_portion();    // 25
let decimal = money.as_decimal();     // 5.25
```

### Soda Value Object
```rust
// Creation
let soda = Soda::new(
    "Coca-Cola".to_string(),
    SodaFlavor::Cola,
    SodaSize::Medium,
    Money::from_dollars_cents(1, 50).unwrap(),
    false, // not diet
    true,  // caffeinated
).unwrap();

// Properties
let name = soda.name();                    // "Coca-Cola"
let flavor = soda.flavor();                // SodaFlavor::Cola
let size = soda.size();                    // SodaSize::Medium
let price = soda.price();                  // Money
let volume = soda.volume_ounces();         // 12

// Operations
let large_soda = soda.with_size(SodaSize::Large, 1.5).unwrap();
let expensive_soda = soda.with_price(new_price).unwrap();
```

### Slot Entity
```rust
// Creation
let mut slot = Slot::new(SlotId::new(1), 20).unwrap();

// Configuration
slot.configure_soda_type(soda).unwrap();

// Inventory management
slot.add_sodas(10).unwrap();
let dispensed = slot.dispense_soda().unwrap();
let removed = slot.remove_sodas(5).unwrap();

// Status
let is_empty = slot.is_empty();
let is_full = slot.is_full();
let remaining = slot.remaining_capacity();
let percentage = slot.fill_percentage();
```

### Soda Machine Aggregate
```rust
// Machine management
let mut machine = SodaMachine::new(SodaMachineId::new(1), 10).unwrap();
machine.add_slot(SlotId::new(1), 20).unwrap();
machine.configure_slot(SlotId::new(1), soda).unwrap();
machine.refill_slot(SlotId::new(1), 10).unwrap();

// Customer operations
machine.insert_money(Money::from_dollars_cents(2, 00).unwrap()).unwrap();
let event = machine.dispense_soda(SlotId::new(1)).unwrap();
machine.return_money().unwrap();

// Administrative
machine.disable();
machine.enable();
let status = machine.status_summary();
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
cargo test
```

The library includes 95 unit tests covering:
- All value object operations
- Entity lifecycle management
- Aggregate orchestration
- Error handling scenarios
- Edge cases and boundary conditions

## 🛡️ Error Handling

All operations return `Result` types for safe error handling:

```rust
match machine.dispense_soda(slot_id) {
    Ok(event) => {
        // Handle successful dispensing
        match event {
            SodaMachineEvent::SodaDispensed { slot_id, soda } => {
                println!("Dispensed: {}", soda.name());
            },
            _ => {}
        }
    },
    Err(SodaMachineError::InsufficientFunds { required, available }) => {
        println!("Need {}, have {}", required, available);
    },
    Err(SodaMachineError::SlotNotFound(slot_id)) => {
        println!("Slot {} not found", slot_id);
    },
    Err(err) => {
        println!("Error: {}", err);
    }
}
```

## 📦 Dependencies

This library has no external dependencies, making it lightweight and fast to compile.

## 🎯 Design Goals

- **Type Safety**: Compile-time guarantees prevent runtime errors
- **Domain Focus**: Business logic is clearly separated from infrastructure
- **Testability**: All components are easily testable in isolation
- **Extensibility**: Easy to add new features while maintaining consistency
- **Performance**: Zero-cost abstractions with efficient memory usage

## 🔧 Development

### Adding New Value Objects
1. Create a new file in `src/domain/value_objects/`
2. Implement the value object with proper validation
3. Add comprehensive unit tests
4. Export the module in `lib.rs`

### Adding New Entities
1. Create a new file in `src/domain/entities/`
2. Implement the entity with identity and lifecycle
3. Add business rule validation
4. Add comprehensive unit tests
5. Export the module in `lib.rs`

### Extending Aggregates
1. Add new methods to existing aggregates
2. Ensure consistency boundaries are maintained
3. Generate appropriate domain events
4. Add comprehensive unit tests

## 📈 Performance Characteristics

- **Compilation**: Fast incremental builds
- **Runtime**: Zero overhead abstractions
- **Memory**: Efficient ownership model
- **Safety**: Compile-time error prevention

---

This library provides a solid foundation for building soda machine applications with clean architecture and robust business logic.
