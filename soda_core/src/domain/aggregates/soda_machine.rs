use std::collections::HashMap;
use std::fmt;
use crate::domain::entities::slot::{Slot, SlotId, SlotError};
use crate::domain::value_objects::soda::Soda;
use crate::domain::value_objects::money::{Money, MoneyError};

/// Represents a soda machine aggregate that orchestrates all soda machine operations
/// This is the main aggregate that maintains consistency across the entire domain
#[derive(Debug, Clone)]
pub struct SodaMachine {
    /// Unique identifier for the soda machine
    id: SodaMachineId,
    /// Collection of slots in the machine
    slots: HashMap<SlotId, Slot>,
    /// Current amount of money inserted by the customer
    inserted_money: Money,
    /// Total amount of money collected by the machine
    total_collected: Money,
    /// Whether the machine is currently operational
    is_operational: bool,
    /// Maximum number of slots this machine can have
    max_slots: u32,
}

/// Unique identifier for a soda machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SodaMachineId(u32);

/// Events that can occur in the soda machine
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Errors that can occur during soda machine operations
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl SodaMachine {
    /// Creates a new soda machine
    /// 
    /// # Arguments
    /// * `id` - Unique identifier for the soda machine
    /// * `max_slots` - Maximum number of slots the machine can have
    /// 
    /// # Returns
    /// * `Result<SodaMachine, SodaMachineError>` - Ok(SodaMachine) if valid, Err if invalid
    /// 
    /// # Examples
    /// ```
    /// use soda_core::domain::aggregates::soda_machine::{SodaMachine, SodaMachineId};
    /// 
    /// let machine = SodaMachine::new(SodaMachineId::new(1), 10).unwrap();
    /// ```
    pub fn new(id: SodaMachineId, max_slots: u32) -> Result<Self, SodaMachineError> {
        if max_slots == 0 {
            return Err(SodaMachineError::InvalidAmount);
        }

        Ok(SodaMachine {
            id,
            slots: HashMap::new(),
            inserted_money: Money::zero(),
            total_collected: Money::zero(),
            is_operational: true,
            max_slots,
        })
    }

    /// Gets the machine ID
    pub fn id(&self) -> SodaMachineId {
        self.id
    }

    /// Gets the current amount of money inserted
    pub fn inserted_money(&self) -> Money {
        self.inserted_money
    }

    /// Gets the total amount of money collected
    pub fn total_collected(&self) -> Money {
        self.total_collected
    }

    /// Checks if the machine is operational
    pub fn is_operational(&self) -> bool {
        self.is_operational
    }

    /// Gets the number of slots in the machine
    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    /// Gets the maximum number of slots
    pub fn max_slots(&self) -> u32 {
        self.max_slots
    }

    /// Gets a slot by ID
    /// 
    /// # Arguments
    /// * `slot_id` - The ID of the slot to retrieve
    /// 
    /// # Returns
    /// * `Option<&Slot>` - Some(slot) if found, None if not found
    pub fn get_slot(&self, slot_id: SlotId) -> Option<&Slot> {
        self.slots.get(&slot_id)
    }

    /// Gets all slots in the machine
    pub fn get_all_slots(&self) -> &HashMap<SlotId, Slot> {
        &self.slots
    }

    /// Adds a new slot to the machine
    /// 
    /// # Arguments
    /// * `slot_id` - The ID for the new slot
    /// * `capacity` - The capacity of the new slot
    /// 
    /// # Returns
    /// * `Result<SodaMachineEvent, SodaMachineError>` - Ok(event) if successful, Err if invalid
    pub fn add_slot(&mut self, slot_id: SlotId, capacity: u32) -> Result<SodaMachineEvent, SodaMachineError> {
        if !self.is_operational {
            return Err(SodaMachineError::MachineNotOperational);
        }

        if self.slots.len() >= self.max_slots as usize {
            return Err(SodaMachineError::TooManySlots);
        }

        if self.slots.contains_key(&slot_id) {
            return Err(SodaMachineError::SlotAlreadyExists(slot_id));
        }

        let slot = Slot::new(slot_id, capacity)
            .map_err(SodaMachineError::SlotError)?;

        self.slots.insert(slot_id, slot);
        Ok(SodaMachineEvent::SlotConfigured { slot_id, soda_type: Soda::new("Empty".to_string(), crate::domain::value_objects::soda::SodaFlavor::Cola, crate::domain::value_objects::soda::SodaSize::Medium, Money::zero(), false, false).unwrap() })
    }

    /// Configures a slot to hold a specific type of soda
    /// 
    /// # Arguments
    /// * `slot_id` - The ID of the slot to configure
    /// * `soda_type` - The type of soda to configure
    /// 
    /// # Returns
    /// * `Result<SodaMachineEvent, SodaMachineError>` - Ok(event) if successful, Err if invalid
    pub fn configure_slot(&mut self, slot_id: SlotId, soda_type: Soda) -> Result<SodaMachineEvent, SodaMachineError> {
        if !self.is_operational {
            return Err(SodaMachineError::MachineNotOperational);
        }

        let slot = self.slots.get_mut(&slot_id)
            .ok_or(SodaMachineError::SlotNotFound(slot_id))?;

        slot.configure_soda_type(soda_type.clone())
            .map_err(SodaMachineError::SlotError)?;

        Ok(SodaMachineEvent::SlotConfigured { slot_id, soda_type })
    }

    /// Refills a slot with sodas
    /// 
    /// # Arguments
    /// * `slot_id` - The ID of the slot to refill
    /// * `quantity` - The number of sodas to add
    /// 
    /// # Returns
    /// * `Result<SodaMachineEvent, SodaMachineError>` - Ok(event) if successful, Err if invalid
    pub fn refill_slot(&mut self, slot_id: SlotId, quantity: u32) -> Result<SodaMachineEvent, SodaMachineError> {
        if !self.is_operational {
            return Err(SodaMachineError::MachineNotOperational);
        }

        let slot = self.slots.get_mut(&slot_id)
            .ok_or(SodaMachineError::SlotNotFound(slot_id))?;

        let added = slot.add_sodas(quantity)
            .map_err(SodaMachineError::SlotError)?;

        Ok(SodaMachineEvent::SlotRefilled { slot_id, quantity_added: added })
    }

    /// Inserts money into the machine
    /// 
    /// # Arguments
    /// * `amount` - The amount of money to insert
    /// 
    /// # Returns
    /// * `Result<SodaMachineEvent, SodaMachineError>` - Ok(event) if successful, Err if invalid
    pub fn insert_money(&mut self, amount: Money) -> Result<SodaMachineEvent, SodaMachineError> {
        if !self.is_operational {
            return Err(SodaMachineError::MachineNotOperational);
        }

        if amount.is_negative() || amount.is_zero() {
            return Err(SodaMachineError::InvalidAmount);
        }

        self.inserted_money = (self.inserted_money + amount)
            .map_err(SodaMachineError::MoneyError)?;

        Ok(SodaMachineEvent::MoneyInserted { 
            amount, 
            total_inserted: self.inserted_money 
        })
    }

    /// Dispenses a soda from a specific slot
    /// 
    /// # Arguments
    /// * `slot_id` - The ID of the slot to dispense from
    /// 
    /// # Returns
    /// * `Result<SodaMachineEvent, SodaMachineError>` - Ok(event) if successful, Err if invalid
    pub fn dispense_soda(&mut self, slot_id: SlotId) -> Result<SodaMachineEvent, SodaMachineError> {
        if !self.is_operational {
            return Err(SodaMachineError::MachineNotOperational);
        }

        let slot = self.slots.get(&slot_id)
            .ok_or(SodaMachineError::SlotNotFound(slot_id))?;

        let soda = slot.soda_type()
            .ok_or(SodaMachineError::SlotError(SlotError::SlotEmpty))?;

        if !slot.can_dispense(soda) {
            return Err(SodaMachineError::SlotError(SlotError::SlotEmpty));
        }

        // Check if customer has enough money
        if self.inserted_money < soda.price() {
            return Err(SodaMachineError::InsufficientFunds {
                required: soda.price(),
                available: self.inserted_money,
            });
        }

        // Dispense the soda
        let slot = self.slots.get_mut(&slot_id).unwrap();
        let dispensed_soda = slot.dispense_soda()
            .map_err(SodaMachineError::SlotError)?;

        // Calculate change
        let change = (self.inserted_money - dispensed_soda.price())
            .map_err(SodaMachineError::MoneyError)?;

        // Update machine state
        self.total_collected = (self.total_collected + dispensed_soda.price())
            .map_err(SodaMachineError::MoneyError)?;
        self.inserted_money = change;

        Ok(SodaMachineEvent::SodaDispensed { slot_id, soda: dispensed_soda })
    }

    /// Returns all inserted money to the customer
    /// 
    /// # Returns
    /// * `Result<SodaMachineEvent, SodaMachineError>` - Ok(event) if successful, Err if invalid
    pub fn return_money(&mut self) -> Result<SodaMachineEvent, SodaMachineError> {
        if !self.is_operational {
            return Err(SodaMachineError::MachineNotOperational);
        }

        if self.inserted_money.is_zero() {
            return Err(SodaMachineError::InvalidAmount);
        }

        let returned_amount = self.inserted_money;
        self.inserted_money = Money::zero();

        Ok(SodaMachineEvent::MoneyReturned { amount: returned_amount })
    }

    /// Returns change to the customer (partial money return)
    /// 
    /// # Arguments
    /// * `amount` - The amount of change to return
    /// 
    /// # Returns
    /// * `Result<SodaMachineEvent, SodaMachineError>` - Ok(event) if successful, Err if invalid
    pub fn return_change(&mut self, amount: Money) -> Result<SodaMachineEvent, SodaMachineError> {
        if !self.is_operational {
            return Err(SodaMachineError::MachineNotOperational);
        }

        if amount.is_negative() || amount.is_zero() {
            return Err(SodaMachineError::InvalidAmount);
        }

        if amount > self.inserted_money {
            return Err(SodaMachineError::InsufficientFunds {
                required: amount,
                available: self.inserted_money,
            });
        }

        self.inserted_money = (self.inserted_money - amount)
            .map_err(SodaMachineError::MoneyError)?;

        Ok(SodaMachineEvent::ChangeReturned { amount })
    }

    /// Enables the soda machine
    /// 
    /// # Returns
    /// * `SodaMachineEvent` - The event that occurred
    pub fn enable(&mut self) -> SodaMachineEvent {
        self.is_operational = true;
        SodaMachineEvent::MachineEnabled
    }

    /// Disables the soda machine
    /// 
    /// # Returns
    /// * `SodaMachineEvent` - The event that occurred
    pub fn disable(&mut self) -> SodaMachineEvent {
        self.is_operational = false;
        SodaMachineEvent::MachineDisabled
    }

    /// Gets the total value of all sodas in the machine
    /// 
    /// # Returns
    /// * `Money` - The total value of all sodas
    pub fn total_inventory_value(&self) -> Money {
        self.slots.values()
            .filter_map(|slot| slot.total_value())
            .fold(Money::zero(), |acc, value| {
                (acc + value).unwrap_or(Money::zero())
            })
    }

    /// Gets the total number of sodas in the machine
    /// 
    /// # Returns
    /// * `u32` - The total number of sodas
    pub fn total_soda_count(&self) -> u32 {
        self.slots.values()
            .map(|slot| slot.quantity())
            .sum()
    }

    /// Gets available sodas (slots that can dispense)
    /// 
    /// # Returns
    /// * `Vec<(SlotId, &Soda)>` - List of available sodas with their slot IDs
    pub fn get_available_sodas(&self) -> Vec<(SlotId, &Soda)> {
        self.slots.iter()
            .filter_map(|(slot_id, slot)| {
                slot.soda_type()
                    .filter(|soda| slot.can_dispense(soda))
                    .map(|soda| (*slot_id, soda))
            })
            .collect()
    }

    /// Checks if a specific soda is available
    /// 
    /// # Arguments
    /// * `soda` - The soda to check for availability
    /// 
    /// # Returns
    /// * `Option<SlotId>` - Some(slot_id) if available, None if not available
    pub fn find_available_soda(&self, soda: &Soda) -> Option<SlotId> {
        self.slots.iter()
            .find(|(_, slot)| slot.can_dispense(soda))
            .map(|(slot_id, _)| *slot_id)
    }

    /// Gets the machine status summary
    /// 
    /// # Returns
    /// * `String` - A summary of the machine's current status
    pub fn status_summary(&self) -> String {
        let available_sodas = self.get_available_sodas().len();
        let total_sodas = self.total_soda_count();
        let total_value = self.total_inventory_value();
        
        format!(
            "Machine {}: {} slots, {} available sodas ({} total), ${:.2} inventory value, ${:.2} inserted, ${:.2} collected - {}",
            self.id.value(),
            self.slot_count(),
            available_sodas,
            total_sodas,
            total_value.as_decimal(),
            self.inserted_money.as_decimal(),
            self.total_collected.as_decimal(),
            if self.is_operational { "Operational" } else { "Out of Service" }
        )
    }
}

impl SodaMachineId {
    /// Creates a new soda machine ID
    /// 
    /// # Arguments
    /// * `id` - The numeric ID
    /// 
    /// # Returns
    /// * `SodaMachineId` - The soda machine ID
    pub fn new(id: u32) -> Self {
        SodaMachineId(id)
    }

    /// Gets the numeric value of the soda machine ID
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for SodaMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.status_summary())
    }
}

impl fmt::Display for SodaMachineId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for SodaMachineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SodaMachineError::SlotNotFound(slot_id) => write!(f, "Slot {} not found", slot_id),
            SodaMachineError::SlotError(err) => write!(f, "Slot error: {}", err),
            SodaMachineError::MoneyError(err) => write!(f, "Money error: {}", err),
            SodaMachineError::InsufficientFunds { required, available } => {
                write!(f, "Insufficient funds: need {}, have {}", required, available)
            },
            SodaMachineError::MachineNotOperational => write!(f, "Machine is not operational"),
            SodaMachineError::InvalidSlotId => write!(f, "Invalid slot ID"),
            SodaMachineError::SlotAlreadyExists(slot_id) => write!(f, "Slot {} already exists", slot_id),
            SodaMachineError::TooManySlots => write!(f, "Too many slots"),
            SodaMachineError::InvalidAmount => write!(f, "Invalid amount"),
        }
    }
}

impl std::error::Error for SodaMachineError {}

impl From<SlotError> for SodaMachineError {
    fn from(err: SlotError) -> Self {
        SodaMachineError::SlotError(err)
    }
}

impl From<MoneyError> for SodaMachineError {
    fn from(err: MoneyError) -> Self {
        SodaMachineError::MoneyError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::soda::{Soda, SodaFlavor, SodaSize};

    fn create_test_soda() -> Soda {
        Soda::new(
            "Coca-Cola".to_string(),
            SodaFlavor::Cola,
            SodaSize::Medium,
            Money::from_dollars_cents(1, 50).unwrap(),
            false,
            true,
        ).unwrap()
    }

    fn create_test_machine() -> SodaMachine {
        SodaMachine::new(SodaMachineId::new(1), 10).unwrap()
    }

    #[test]
    fn test_machine_creation() {
        let machine = create_test_machine();
        
        assert_eq!(machine.id().value(), 1);
        assert_eq!(machine.slot_count(), 0);
        assert_eq!(machine.max_slots(), 10);
        assert!(machine.is_operational());
        assert_eq!(machine.inserted_money(), Money::zero());
        assert_eq!(machine.total_collected(), Money::zero());
    }

    #[test]
    fn test_machine_creation_zero_max_slots() {
        let result = SodaMachine::new(SodaMachineId::new(1), 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaMachineError::InvalidAmount);
    }

    #[test]
    fn test_add_slot() {
        let mut machine = create_test_machine();
        let event = machine.add_slot(SlotId::new(1), 20).unwrap();
        
        assert_eq!(machine.slot_count(), 1);
        assert!(machine.get_slot(SlotId::new(1)).is_some());
        
        match event {
            SodaMachineEvent::SlotConfigured { slot_id, .. } => {
                assert_eq!(slot_id, SlotId::new(1));
            },
            _ => panic!("Expected SlotConfigured event"),
        }
    }

    #[test]
    fn test_add_slot_duplicate() {
        let mut machine = create_test_machine();
        machine.add_slot(SlotId::new(1), 20).unwrap();
        
        let result = machine.add_slot(SlotId::new(1), 15);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaMachineError::SlotAlreadyExists(SlotId::new(1)));
    }

    #[test]
    fn test_add_slot_too_many() {
        let mut machine = SodaMachine::new(SodaMachineId::new(1), 1).unwrap();
        machine.add_slot(SlotId::new(1), 20).unwrap();
        
        let result = machine.add_slot(SlotId::new(2), 15);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaMachineError::TooManySlots);
    }

    #[test]
    fn test_configure_slot() {
        let mut machine = create_test_machine();
        machine.add_slot(SlotId::new(1), 20).unwrap();
        
        let soda = create_test_soda();
        let event = machine.configure_slot(SlotId::new(1), soda.clone()).unwrap();
        
        let slot = machine.get_slot(SlotId::new(1)).unwrap();
        assert!(slot.soda_type().is_some());
        assert_eq!(slot.soda_type().unwrap().name(), "Coca-Cola");
        
        match event {
            SodaMachineEvent::SlotConfigured { slot_id, soda_type } => {
                assert_eq!(slot_id, SlotId::new(1));
                assert_eq!(soda_type.name(), "Coca-Cola");
            },
            _ => panic!("Expected SlotConfigured event"),
        }
    }

    #[test]
    fn test_configure_slot_not_found() {
        let mut machine = create_test_machine();
        let soda = create_test_soda();
        
        let result = machine.configure_slot(SlotId::new(1), soda);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaMachineError::SlotNotFound(SlotId::new(1)));
    }

    #[test]
    fn test_refill_slot() {
        let mut machine = create_test_machine();
        machine.add_slot(SlotId::new(1), 20).unwrap();
        machine.configure_slot(SlotId::new(1), create_test_soda()).unwrap();
        
        let event = machine.refill_slot(SlotId::new(1), 10).unwrap();
        
        let slot = machine.get_slot(SlotId::new(1)).unwrap();
        assert_eq!(slot.quantity(), 10);
        
        match event {
            SodaMachineEvent::SlotRefilled { slot_id, quantity_added } => {
                assert_eq!(slot_id, SlotId::new(1));
                assert_eq!(quantity_added, 10);
            },
            _ => panic!("Expected SlotRefilled event"),
        }
    }

    #[test]
    fn test_insert_money() {
        let mut machine = create_test_machine();
        let amount = Money::from_dollars_cents(2, 00).unwrap();
        
        let event = machine.insert_money(amount).unwrap();
        
        assert_eq!(machine.inserted_money(), amount);
        
        match event {
            SodaMachineEvent::MoneyInserted { amount: event_amount, total_inserted } => {
                assert_eq!(event_amount, amount);
                assert_eq!(total_inserted, amount);
            },
            _ => panic!("Expected MoneyInserted event"),
        }
    }

    #[test]
    fn test_insert_money_negative() {
        let mut machine = create_test_machine();
        let amount = Money::from_dollars_cents(-1, 00).unwrap();
        
        let result = machine.insert_money(amount);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaMachineError::InvalidAmount);
    }

    #[test]
    fn test_dispense_soda() {
        let mut machine = create_test_machine();
        machine.add_slot(SlotId::new(1), 20).unwrap();
        machine.configure_slot(SlotId::new(1), create_test_soda()).unwrap();
        machine.refill_slot(SlotId::new(1), 5).unwrap();
        machine.insert_money(Money::from_dollars_cents(2, 00).unwrap()).unwrap();
        
        let event = machine.dispense_soda(SlotId::new(1)).unwrap();
        
        assert_eq!(machine.inserted_money(), Money::from_dollars_cents(0, 50).unwrap()); // $2.00 - $1.50
        assert_eq!(machine.total_collected(), Money::from_dollars_cents(1, 50).unwrap());
        
        let slot = machine.get_slot(SlotId::new(1)).unwrap();
        assert_eq!(slot.quantity(), 4); // 5 - 1
        
        match event {
            SodaMachineEvent::SodaDispensed { slot_id, soda } => {
                assert_eq!(slot_id, SlotId::new(1));
                assert_eq!(soda.name(), "Coca-Cola");
            },
            _ => panic!("Expected SodaDispensed event"),
        }
    }

    #[test]
    fn test_dispense_soda_insufficient_funds() {
        let mut machine = create_test_machine();
        machine.add_slot(SlotId::new(1), 20).unwrap();
        machine.configure_slot(SlotId::new(1), create_test_soda()).unwrap();
        machine.refill_slot(SlotId::new(1), 5).unwrap();
        machine.insert_money(Money::from_dollars_cents(1, 00).unwrap()).unwrap(); // Not enough
        
        let result = machine.dispense_soda(SlotId::new(1));
        assert!(result.is_err());
        
        match result.unwrap_err() {
            SodaMachineError::InsufficientFunds { required, available } => {
                assert_eq!(required, Money::from_dollars_cents(1, 50).unwrap());
                assert_eq!(available, Money::from_dollars_cents(1, 00).unwrap());
            },
            _ => panic!("Expected InsufficientFunds error"),
        }
    }

    #[test]
    fn test_dispense_soda_empty_slot() {
        let mut machine = create_test_machine();
        machine.add_slot(SlotId::new(1), 20).unwrap();
        machine.configure_slot(SlotId::new(1), create_test_soda()).unwrap();
        // Don't refill the slot
        machine.insert_money(Money::from_dollars_cents(2, 00).unwrap()).unwrap();
        
        let result = machine.dispense_soda(SlotId::new(1));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaMachineError::SlotError(SlotError::SlotEmpty));
    }

    #[test]
    fn test_return_money() {
        let mut machine = create_test_machine();
        machine.insert_money(Money::from_dollars_cents(2, 00).unwrap()).unwrap();
        
        let event = machine.return_money().unwrap();
        
        assert_eq!(machine.inserted_money(), Money::zero());
        
        match event {
            SodaMachineEvent::MoneyReturned { amount } => {
                assert_eq!(amount, Money::from_dollars_cents(2, 00).unwrap());
            },
            _ => panic!("Expected MoneyReturned event"),
        }
    }

    #[test]
    fn test_return_money_none_inserted() {
        let mut machine = create_test_machine();
        
        let result = machine.return_money();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaMachineError::InvalidAmount);
    }

    #[test]
    fn test_return_change() {
        let mut machine = create_test_machine();
        machine.insert_money(Money::from_dollars_cents(2, 00).unwrap()).unwrap();
        
        let change_amount = Money::from_dollars_cents(0, 50).unwrap();
        let event = machine.return_change(change_amount).unwrap();
        
        assert_eq!(machine.inserted_money(), Money::from_dollars_cents(1, 50).unwrap());
        
        match event {
            SodaMachineEvent::ChangeReturned { amount } => {
                assert_eq!(amount, change_amount);
            },
            _ => panic!("Expected ChangeReturned event"),
        }
    }

    #[test]
    fn test_enable_disable() {
        let mut machine = create_test_machine();
        assert!(machine.is_operational());
        
        let event = machine.disable();
        assert!(!machine.is_operational());
        
        match event {
            SodaMachineEvent::MachineDisabled => {},
            _ => panic!("Expected MachineDisabled event"),
        }
        
        let event = machine.enable();
        assert!(machine.is_operational());
        
        match event {
            SodaMachineEvent::MachineEnabled => {},
            _ => panic!("Expected MachineEnabled event"),
        }
    }

    #[test]
    fn test_total_inventory_value() {
        let mut machine = create_test_machine();
        machine.add_slot(SlotId::new(1), 20).unwrap();
        machine.configure_slot(SlotId::new(1), create_test_soda()).unwrap();
        machine.refill_slot(SlotId::new(1), 5).unwrap();
        
        let total_value = machine.total_inventory_value();
        assert_eq!(total_value, Money::from_dollars_cents(7, 50).unwrap()); // $1.50 * 5
    }

    #[test]
    fn test_total_soda_count() {
        let mut machine = create_test_machine();
        machine.add_slot(SlotId::new(1), 20).unwrap();
        machine.configure_slot(SlotId::new(1), create_test_soda()).unwrap();
        machine.refill_slot(SlotId::new(1), 5).unwrap();
        
        assert_eq!(machine.total_soda_count(), 5);
    }

    #[test]
    fn test_get_available_sodas() {
        let mut machine = create_test_machine();
        machine.add_slot(SlotId::new(1), 20).unwrap();
        machine.configure_slot(SlotId::new(1), create_test_soda()).unwrap();
        machine.refill_slot(SlotId::new(1), 5).unwrap();
        
        let available = machine.get_available_sodas();
        assert_eq!(available.len(), 1);
        assert_eq!(available[0].0, SlotId::new(1));
        assert_eq!(available[0].1.name(), "Coca-Cola");
    }

    #[test]
    fn test_find_available_soda() {
        let mut machine = create_test_machine();
        machine.add_slot(SlotId::new(1), 20).unwrap();
        machine.configure_slot(SlotId::new(1), create_test_soda()).unwrap();
        machine.refill_slot(SlotId::new(1), 5).unwrap();
        
        let soda = create_test_soda();
        let slot_id = machine.find_available_soda(&soda);
        assert_eq!(slot_id, Some(SlotId::new(1)));
    }

    #[test]
    fn test_operations_when_disabled() {
        let mut machine = create_test_machine();
        machine.disable();
        
        let result = machine.add_slot(SlotId::new(1), 20);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaMachineError::MachineNotOperational);
        
        let result = machine.insert_money(Money::from_dollars_cents(1, 00).unwrap());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaMachineError::MachineNotOperational);
    }

    #[test]
    fn test_soda_machine_id() {
        let id = SodaMachineId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_soda_machine_id_ordering() {
        let id1 = SodaMachineId::new(1);
        let id2 = SodaMachineId::new(2);
        let id3 = SodaMachineId::new(1);
        
        assert!(id1 < id2);
        assert!(id1 == id3);
        assert!(id2 > id1);
    }

    #[test]
    fn test_status_summary() {
        let mut machine = create_test_machine();
        machine.add_slot(SlotId::new(1), 20).unwrap();
        machine.configure_slot(SlotId::new(1), create_test_soda()).unwrap();
        machine.refill_slot(SlotId::new(1), 5).unwrap();
        machine.insert_money(Money::from_dollars_cents(2, 00).unwrap()).unwrap();
        
        let summary = machine.status_summary();
        assert!(summary.contains("Machine 1"));
        assert!(summary.contains("1 slots"));
        assert!(summary.contains("1 available sodas"));
        assert!(summary.contains("5 total"));
        assert!(summary.contains("Operational"));
    }

    #[test]
    fn test_display() {
        let machine = create_test_machine();
        let display = format!("{}", machine);
        assert!(display.contains("Machine 1"));
        assert!(display.contains("Operational"));
    }
}
