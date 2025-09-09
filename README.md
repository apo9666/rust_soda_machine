# Rust Soda Machine

A comprehensive domain-driven design implementation of a soda machine system built in Rust. This project demonstrates clean architecture principles, type safety, and robust business logic implementation.

## 🏗️ Architecture

This project follows **Domain-Driven Design (DDD)** principles with a clean separation of concerns:

```
soda_core/
├── src/
│   └── domain/
│       ├── value_objects/     # Immutable value objects
│       │   ├── money.rs       # Monetary calculations
│       │   └── soda.rs        # Soda product definitions
│       ├── entities/          # Objects with identity
│       │   └── slot.rs        # Inventory slot management
│       └── aggregates/        # Consistency boundaries
│           └── soda_machine.rs # Main business orchestrator
```

## 🚀 Features

### 💰 Money Value Object
- **Precision-safe operations** using cents to avoid floating-point issues
- **Mathematical operations**: addition, subtraction, multiplication, division
- **Overflow/underflow protection** with checked arithmetic
- **Multiple constructors**: from dollars/cents, decimal amounts, or cents
- **Rich validation** and comprehensive error handling

### 🥤 Soda Value Object
- **10 different flavors**: Cola, Orange, Lemon-Lime, Root Beer, Grape, Cherry, Vanilla, Strawberry, Peach, Watermelon
- **4 sizes**: Small (8oz), Medium (12oz), Large (16oz), X-Large (20oz)
- **Product properties**: Diet status, caffeine content, pricing
- **Size conversion** with price adjustment capabilities
- **String parsing** for flavors and sizes

### 📦 Slot Entity
- **Inventory management** with unique slot identification
- **Soda type configuration** and quantity tracking
- **Capacity management** with validation rules
- **Dispensing operations** with business rule enforcement
- **Operational status** control (enable/disable)
- **Value calculations** for inventory worth

### 🏪 Soda Machine Aggregate
- **Slot orchestration** with capacity limits and management
- **Customer operations**: money insertion, soda dispensing, change calculation
- **Payment processing** with fund validation
- **Administrative functions**: slot configuration, refilling, machine control
- **Domain events** for external system integration
- **Comprehensive status monitoring** and reporting

## 🧪 Testing

The project includes **95 comprehensive unit tests** covering:

- ✅ All value object operations and edge cases
- ✅ Entity lifecycle and business rule enforcement
- ✅ Aggregate orchestration and consistency
- ✅ Error handling and validation scenarios
- ✅ Integration between domain objects

Run tests with:
```bash
cargo test --package soda_core
```

## 📦 Usage

### Basic Setup

```rust
use soda_core::domain::aggregates::soda_machine::{SodaMachine, SodaMachineId};
use soda_core::domain::entities::slot::SlotId;
use soda_core::domain::value_objects::soda::{Soda, SodaFlavor, SodaSize};
use soda_core::domain::value_objects::money::Money;

// Create a new soda machine
let mut machine = SodaMachine::new(SodaMachineId::new(1), 10).unwrap();
```

### Configure Machine

```rust
// Add slots
machine.add_slot(SlotId::new(1), 20).unwrap();
machine.add_slot(SlotId::new(2), 15).unwrap();

// Configure with soda types
let coke = Soda::new(
    "Coca-Cola".to_string(),
    SodaFlavor::Cola,
    SodaSize::Medium,
    Money::from_dollars_cents(1, 50).unwrap(),
    false, // not diet
    true,  // caffeinated
).unwrap();

machine.configure_slot(SlotId::new(1), coke).unwrap();

// Refill inventory
machine.refill_slot(SlotId::new(1), 10).unwrap();
```

### Customer Operations

```rust
// Customer inserts money
machine.insert_money(Money::from_dollars_cents(2, 00).unwrap()).unwrap();

// Dispense a soda
let event = machine.dispense_soda(SlotId::new(1)).unwrap();

match event {
    SodaMachineEvent::SodaDispensed { slot_id, soda } => {
        println!("Dispensed {} from slot {}", soda.name(), slot_id);
        // Change is automatically calculated
    },
    _ => {}
}

// Check remaining money
println!("Remaining: {}", machine.inserted_money()); // $0.50

// Return all money if needed
machine.return_money().unwrap();
```

### Administrative Operations

```rust
// Check machine status
println!("{}", machine.status_summary());
// "Machine 1: 2 slots, 1 available sodas (10 total), $15.00 inventory value, $0.00 inserted, $1.50 collected - Operational"

// Get available sodas
let available = machine.get_available_sodas();
for (slot_id, soda) in available {
    println!("Slot {}: {} - ${}", slot_id, soda.name(), soda.price());
}

// Administrative control
machine.disable(); // Put machine out of service
machine.enable();  // Bring back online
```

## 🎯 Domain Events

The system generates rich domain events for external integration:

```rust
pub enum SodaMachineEvent {
    MoneyInserted { amount: Money, total_inserted: Money },
    MoneyReturned { amount: Money },
    SodaDispensed { slot_id: SlotId, soda: Soda },
    SlotConfigured { slot_id: SlotId, soda_type: Soda },
    SlotRefilled { slot_id: SlotId, quantity_added: u32 },
    MachineEnabled,
    MachineDisabled,
    ChangeReturned { amount: Money },
}
```

## 🛡️ Error Handling

Comprehensive error handling with descriptive messages:

```rust
pub enum SodaMachineError {
    SlotNotFound(SlotId),
    SlotError(SlotError),
    MoneyError(MoneyError),
    InsufficientFunds { required: Money, available: Money },
    MachineNotOperational,
    InvalidSlotId,
    SlotAlreadyExists(SlotId),
    TooManySlots,
    InvalidAmount,
}
```

## 🏃‍♂️ Getting Started

### Prerequisites
- Rust 1.70+ (2024 edition)
- Cargo

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd rust_soda_machine
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

4. Run with examples:
```bash
cargo run --example soda_machine_demo
```

## 📚 Design Principles

### Domain-Driven Design
- **Value Objects**: Immutable objects representing concepts (Money, Soda)
- **Entities**: Objects with identity and lifecycle (Slot)
- **Aggregates**: Consistency boundaries (SodaMachine)
- **Domain Events**: Communication between bounded contexts

### Type Safety
- **Zero-cost abstractions** with compile-time guarantees
- **Rich type system** preventing invalid operations
- **Ownership model** ensuring memory safety
- **Pattern matching** for exhaustive error handling

### Business Logic
- **Encapsulation** of business rules within domain objects
- **Validation** at domain boundaries
- **Consistency** maintained through aggregate patterns
- **Event sourcing** ready architecture

## 🔧 Development

### Project Structure
```
rust_soda_machine/
├── Cargo.toml              # Workspace configuration
├── soda_core/              # Core domain library
│   ├── Cargo.toml          # Library dependencies
│   └── src/
│       └── domain/         # Domain layer
│           ├── value_objects/
│           ├── entities/
│           └── aggregates/
└── README.md               # This file
```

### Adding New Features
1. **Value Objects**: Add new concepts as immutable value objects
2. **Entities**: Add new objects with identity and lifecycle
3. **Aggregates**: Extend existing aggregates or create new ones
4. **Events**: Add new domain events for significant operations

### Testing Strategy
- **Unit Tests**: Test individual domain objects in isolation
- **Integration Tests**: Test interactions between domain objects
- **Property Tests**: Test business rules with generated data
- **Edge Cases**: Test error conditions and boundary values

## 📈 Performance

- **Zero runtime overhead** for type safety
- **Efficient memory usage** with Rust's ownership model
- **Fast compilation** with incremental builds
- **Optimized operations** with checked arithmetic

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Domain-Driven Design patterns and principles
- Rust language team for excellent tooling
- Clean Architecture principles
- Test-Driven Development practices

---

**Built with ❤️ in Rust**
