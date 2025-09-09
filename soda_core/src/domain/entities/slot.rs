use std::fmt;
use crate::domain::value_objects::soda::Soda;
use crate::domain::value_objects::money::Money;

/// Represents a slot in the soda machine that can hold sodas
/// This is an entity with identity and lifecycle
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slot {
    /// Unique identifier for the slot
    id: SlotId,
    /// The type of soda this slot is configured for
    soda_type: Option<Soda>,
    /// Current quantity of sodas in the slot
    quantity: u32,
    /// Maximum capacity of the slot
    max_capacity: u32,
    /// Whether the slot is currently enabled/operational
    is_enabled: bool,
}

/// Unique identifier for a slot
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SlotId(u32);

/// Errors that can occur during slot operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotError {
    InvalidQuantity(String),
    InvalidCapacity(String),
    SlotEmpty,
    SlotFull,
    SlotDisabled,
    SodaTypeMismatch,
    InsufficientQuantity,
    InvalidSlotId,
}

impl Slot {
    /// Creates a new empty slot
    /// 
    /// # Arguments
    /// * `id` - Unique identifier for the slot
    /// * `max_capacity` - Maximum number of sodas the slot can hold
    /// 
    /// # Returns
    /// * `Result<Slot, SlotError>` - Ok(Slot) if valid, Err if invalid
    /// 
    /// # Examples
    /// ```
    /// use soda_core::domain::entities::slot::{Slot, SlotId};
    /// 
    /// let slot = Slot::new(SlotId::new(1), 20).unwrap();
    /// ```
    pub fn new(id: SlotId, max_capacity: u32) -> Result<Self, SlotError> {
        if max_capacity == 0 {
            return Err(SlotError::InvalidCapacity("Capacity must be greater than 0".to_string()));
        }

        Ok(Slot {
            id,
            soda_type: None,
            quantity: 0,
            max_capacity,
            is_enabled: true,
        })
    }

    /// Creates a new slot with a specific soda type and quantity
    /// 
    /// # Arguments
    /// * `id` - Unique identifier for the slot
    /// * `soda_type` - The type of soda this slot will hold
    /// * `quantity` - Initial quantity of sodas
    /// * `max_capacity` - Maximum number of sodas the slot can hold
    /// 
    /// # Returns
    /// * `Result<Slot, SlotError>` - Ok(Slot) if valid, Err if invalid
    pub fn new_with_soda(
        id: SlotId,
        soda_type: Soda,
        quantity: u32,
        max_capacity: u32,
    ) -> Result<Self, SlotError> {
        if max_capacity == 0 {
            return Err(SlotError::InvalidCapacity("Capacity must be greater than 0".to_string()));
        }

        if quantity > max_capacity {
            return Err(SlotError::InvalidQuantity("Quantity cannot exceed capacity".to_string()));
        }

        Ok(Slot {
            id,
            soda_type: Some(soda_type),
            quantity,
            max_capacity,
            is_enabled: true,
        })
    }

    /// Gets the slot ID
    pub fn id(&self) -> SlotId {
        self.id
    }

    /// Gets the soda type configured for this slot
    pub fn soda_type(&self) -> Option<&Soda> {
        self.soda_type.as_ref()
    }

    /// Gets the current quantity of sodas in the slot
    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    /// Gets the maximum capacity of the slot
    pub fn max_capacity(&self) -> u32 {
        self.max_capacity
    }

    /// Checks if the slot is enabled
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    /// Checks if the slot is empty
    pub fn is_empty(&self) -> bool {
        self.quantity == 0
    }

    /// Checks if the slot is full
    pub fn is_full(&self) -> bool {
        self.quantity >= self.max_capacity
    }

    /// Gets the remaining capacity of the slot
    pub fn remaining_capacity(&self) -> u32 {
        self.max_capacity.saturating_sub(self.quantity)
    }

    /// Gets the fill percentage of the slot (0.0 to 1.0)
    pub fn fill_percentage(&self) -> f64 {
        if self.max_capacity == 0 {
            0.0
        } else {
            self.quantity as f64 / self.max_capacity as f64
        }
    }

    /// Configures the slot to hold a specific type of soda
    /// 
    /// # Arguments
    /// * `soda_type` - The type of soda to configure
    /// 
    /// # Returns
    /// * `Result<(), SlotError>` - Ok if successful, Err if slot is not empty
    pub fn configure_soda_type(&mut self, soda_type: Soda) -> Result<(), SlotError> {
        if !self.is_empty() {
            return Err(SlotError::SodaTypeMismatch);
        }

        self.soda_type = Some(soda_type);
        Ok(())
    }

    /// Adds sodas to the slot
    /// 
    /// # Arguments
    /// * `count` - Number of sodas to add
    /// 
    /// # Returns
    /// * `Result<u32, SlotError>` - Ok(actual_added) if successful, Err if invalid
    pub fn add_sodas(&mut self, count: u32) -> Result<u32, SlotError> {
        if !self.is_enabled {
            return Err(SlotError::SlotDisabled);
        }

        if count == 0 {
            return Ok(0);
        }

        let available_space = self.remaining_capacity();
        let actual_added = count.min(available_space);

        self.quantity += actual_added;

        if actual_added < count {
            Err(SlotError::SlotFull)
        } else {
            Ok(actual_added)
        }
    }

    /// Removes sodas from the slot
    /// 
    /// # Arguments
    /// * `count` - Number of sodas to remove
    /// 
    /// # Returns
    /// * `Result<u32, SlotError>` - Ok(actual_removed) if successful, Err if invalid
    pub fn remove_sodas(&mut self, count: u32) -> Result<u32, SlotError> {
        if !self.is_enabled {
            return Err(SlotError::SlotDisabled);
        }

        if count == 0 {
            return Ok(0);
        }

        if self.is_empty() {
            return Err(SlotError::SlotEmpty);
        }

        let actual_removed = count.min(self.quantity);
        self.quantity -= actual_removed;

        Ok(actual_removed)
    }

    /// Dispenses a single soda from the slot
    /// 
    /// # Returns
    /// * `Result<Soda, SlotError>` - Ok(soda) if successful, Err if cannot dispense
    pub fn dispense_soda(&mut self) -> Result<Soda, SlotError> {
        if !self.is_enabled {
            return Err(SlotError::SlotDisabled);
        }

        if self.is_empty() {
            return Err(SlotError::SlotEmpty);
        }

        self.quantity -= 1;
        Ok(self.soda_type.clone().unwrap())
    }

    /// Enables the slot
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    /// Disables the slot
    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    /// Updates the maximum capacity of the slot
    /// 
    /// # Arguments
    /// * `new_capacity` - New maximum capacity
    /// 
    /// # Returns
    /// * `Result<(), SlotError>` - Ok if successful, Err if invalid
    pub fn set_capacity(&mut self, new_capacity: u32) -> Result<(), SlotError> {
        if new_capacity == 0 {
            return Err(SlotError::InvalidCapacity("Capacity must be greater than 0".to_string()));
        }

        if self.quantity > new_capacity {
            return Err(SlotError::InvalidCapacity("Cannot reduce capacity below current quantity".to_string()));
        }

        self.max_capacity = new_capacity;
        Ok(())
    }

    /// Gets the total value of sodas in the slot
    /// 
    /// # Returns
    /// * `Option<Money>` - Some(total_value) if slot has soda type, None if empty
    pub fn total_value(&self) -> Option<Money> {
        if let Some(soda_type) = &self.soda_type {
            if self.quantity > 0 {
                match soda_type.price() * (self.quantity as i64) {
                    Ok(total) => Some(total),
                    Err(_) => None,
                }
            } else {
                Some(Money::zero())
            }
        } else {
            None
        }
    }

    /// Checks if the slot can dispense the requested soda type
    /// 
    /// # Arguments
    /// * `soda` - The soda type to check
    /// 
    /// # Returns
    /// * `bool` - True if the slot can dispense this soda type
    pub fn can_dispense(&self, soda: &Soda) -> bool {
        self.is_enabled && 
        !self.is_empty() && 
        self.soda_type.as_ref().map_or(false, |slot_soda| slot_soda.is_same_type(soda))
    }
}

impl SlotId {
    /// Creates a new slot ID
    /// 
    /// # Arguments
    /// * `id` - The numeric ID
    /// 
    /// # Returns
    /// * `SlotId` - The slot ID
    pub fn new(id: u32) -> Self {
        SlotId(id)
    }

    /// Gets the numeric value of the slot ID
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(soda_type) = &self.soda_type {
            write!(
                f,
                "Slot {}: {} ({} of {}) - {}",
                self.id.value(),
                soda_type.name(),
                self.quantity,
                self.max_capacity,
                if self.is_enabled { "Enabled" } else { "Disabled" }
            )
        } else {
            write!(
                f,
                "Slot {}: Empty ({} of {}) - {}",
                self.id.value(),
                self.quantity,
                self.max_capacity,
                if self.is_enabled { "Enabled" } else { "Disabled" }
            )
        }
    }
}

impl fmt::Display for SlotId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for SlotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlotError::InvalidQuantity(msg) => write!(f, "Invalid quantity: {}", msg),
            SlotError::InvalidCapacity(msg) => write!(f, "Invalid capacity: {}", msg),
            SlotError::SlotEmpty => write!(f, "Slot is empty"),
            SlotError::SlotFull => write!(f, "Slot is full"),
            SlotError::SlotDisabled => write!(f, "Slot is disabled"),
            SlotError::SodaTypeMismatch => write!(f, "Soda type mismatch"),
            SlotError::InsufficientQuantity => write!(f, "Insufficient quantity"),
            SlotError::InvalidSlotId => write!(f, "Invalid slot ID"),
        }
    }
}

impl std::error::Error for SlotError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::soda::{Soda, SodaFlavor, SodaSize};

    fn create_test_soda() -> Soda {
        Soda::new(
            "Coca-Cola".to_string(),
            SodaFlavor::Cola,
            SodaSize::Medium,
            crate::domain::value_objects::money::Money::from_dollars_cents(1, 50).unwrap(),
            false,
            true,
        ).unwrap()
    }

    #[test]
    fn test_slot_creation() {
        let slot = Slot::new(SlotId::new(1), 20).unwrap();
        
        assert_eq!(slot.id().value(), 1);
        assert_eq!(slot.quantity(), 0);
        assert_eq!(slot.max_capacity(), 20);
        assert!(slot.is_enabled());
        assert!(slot.is_empty());
        assert!(!slot.is_full());
        assert_eq!(slot.remaining_capacity(), 20);
        assert_eq!(slot.fill_percentage(), 0.0);
    }

    #[test]
    fn test_slot_creation_with_soda() {
        let soda = create_test_soda();
        let slot = Slot::new_with_soda(SlotId::new(1), soda.clone(), 10, 20).unwrap();
        
        assert_eq!(slot.id().value(), 1);
        assert_eq!(slot.quantity(), 10);
        assert_eq!(slot.max_capacity(), 20);
        assert!(slot.soda_type().is_some());
        assert_eq!(slot.soda_type().unwrap().name(), "Coca-Cola");
        assert!(!slot.is_empty());
        assert!(!slot.is_full());
        assert_eq!(slot.remaining_capacity(), 10);
        assert_eq!(slot.fill_percentage(), 0.5);
    }

    #[test]
    fn test_slot_creation_zero_capacity() {
        let result = Slot::new(SlotId::new(1), 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SlotError::InvalidCapacity("Capacity must be greater than 0".to_string()));
    }

    #[test]
    fn test_slot_creation_quantity_exceeds_capacity() {
        let soda = create_test_soda();
        let result = Slot::new_with_soda(SlotId::new(1), soda, 25, 20);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SlotError::InvalidQuantity("Quantity cannot exceed capacity".to_string()));
    }

    #[test]
    fn test_configure_soda_type() {
        let mut slot = Slot::new(SlotId::new(1), 20).unwrap();
        let soda = create_test_soda();
        
        slot.configure_soda_type(soda.clone()).unwrap();
        assert!(slot.soda_type().is_some());
        assert_eq!(slot.soda_type().unwrap().name(), "Coca-Cola");
    }

    #[test]
    fn test_configure_soda_type_when_not_empty() {
        let soda = create_test_soda();
        let mut slot = Slot::new_with_soda(SlotId::new(1), soda.clone(), 5, 20).unwrap();
        
        let result = slot.configure_soda_type(soda);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SlotError::SodaTypeMismatch);
    }

    #[test]
    fn test_add_sodas() {
        let mut slot = Slot::new(SlotId::new(1), 20).unwrap();
        
        let added = slot.add_sodas(5).unwrap();
        assert_eq!(added, 5);
        assert_eq!(slot.quantity(), 5);
        assert_eq!(slot.remaining_capacity(), 15);
    }

    #[test]
    fn test_add_sodas_exceeds_capacity() {
        let mut slot = Slot::new(SlotId::new(1), 20).unwrap();
        slot.add_sodas(15).unwrap();
        
        let result = slot.add_sodas(10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SlotError::SlotFull);
        assert_eq!(slot.quantity(), 20); // Should be at capacity
    }

    #[test]
    fn test_add_sodas_to_disabled_slot() {
        let mut slot = Slot::new(SlotId::new(1), 20).unwrap();
        slot.disable();
        
        let result = slot.add_sodas(5);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SlotError::SlotDisabled);
    }

    #[test]
    fn test_remove_sodas() {
        let soda = create_test_soda();
        let mut slot = Slot::new_with_soda(SlotId::new(1), soda, 10, 20).unwrap();
        
        let removed = slot.remove_sodas(3).unwrap();
        assert_eq!(removed, 3);
        assert_eq!(slot.quantity(), 7);
    }

    #[test]
    fn test_remove_sodas_from_empty_slot() {
        let mut slot = Slot::new(SlotId::new(1), 20).unwrap();
        
        let result = slot.remove_sodas(5);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SlotError::SlotEmpty);
    }

    #[test]
    fn test_remove_sodas_from_disabled_slot() {
        let soda = create_test_soda();
        let mut slot = Slot::new_with_soda(SlotId::new(1), soda, 10, 20).unwrap();
        slot.disable();
        
        let result = slot.remove_sodas(5);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SlotError::SlotDisabled);
    }

    #[test]
    fn test_dispense_soda() {
        let soda = create_test_soda();
        let mut slot = Slot::new_with_soda(SlotId::new(1), soda.clone(), 5, 20).unwrap();
        
        let dispensed = slot.dispense_soda().unwrap();
        assert_eq!(dispensed.name(), "Coca-Cola");
        assert_eq!(slot.quantity(), 4);
    }

    #[test]
    fn test_dispense_soda_from_empty_slot() {
        let mut slot = Slot::new(SlotId::new(1), 20).unwrap();
        
        let result = slot.dispense_soda();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SlotError::SlotEmpty);
    }

    #[test]
    fn test_dispense_soda_from_disabled_slot() {
        let soda = create_test_soda();
        let mut slot = Slot::new_with_soda(SlotId::new(1), soda, 5, 20).unwrap();
        slot.disable();
        
        let result = slot.dispense_soda();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SlotError::SlotDisabled);
    }

    #[test]
    fn test_enable_disable() {
        let mut slot = Slot::new(SlotId::new(1), 20).unwrap();
        assert!(slot.is_enabled());
        
        slot.disable();
        assert!(!slot.is_enabled());
        
        slot.enable();
        assert!(slot.is_enabled());
    }

    #[test]
    fn test_set_capacity() {
        let mut slot = Slot::new(SlotId::new(1), 20).unwrap();
        
        slot.set_capacity(30).unwrap();
        assert_eq!(slot.max_capacity(), 30);
    }

    #[test]
    fn test_set_capacity_zero() {
        let mut slot = Slot::new(SlotId::new(1), 20).unwrap();
        
        let result = slot.set_capacity(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SlotError::InvalidCapacity("Capacity must be greater than 0".to_string()));
    }

    #[test]
    fn test_set_capacity_below_quantity() {
        let soda = create_test_soda();
        let mut slot = Slot::new_with_soda(SlotId::new(1), soda, 10, 20).unwrap();
        
        let result = slot.set_capacity(5);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SlotError::InvalidCapacity("Cannot reduce capacity below current quantity".to_string()));
    }

    #[test]
    fn test_total_value() {
        let soda = create_test_soda();
        let slot = Slot::new_with_soda(SlotId::new(1), soda, 5, 20).unwrap();
        
        let total_value = slot.total_value().unwrap();
        assert_eq!(total_value, crate::domain::value_objects::money::Money::from_dollars_cents(7, 50).unwrap()); // $1.50 * 5
    }

    #[test]
    fn test_total_value_empty_slot() {
        let slot = Slot::new(SlotId::new(1), 20).unwrap();
        assert!(slot.total_value().is_none());
    }

    #[test]
    fn test_can_dispense() {
        let soda = create_test_soda();
        let slot = Slot::new_with_soda(SlotId::new(1), soda.clone(), 5, 20).unwrap();
        
        assert!(slot.can_dispense(&soda));
    }

    #[test]
    fn test_can_dispense_different_soda() {
        let soda1 = create_test_soda();
        let soda2 = Soda::new(
            "Pepsi".to_string(),
            SodaFlavor::Cola,
            SodaSize::Medium,
            crate::domain::value_objects::money::Money::from_dollars_cents(1, 50).unwrap(),
            false,
            true,
        ).unwrap();
        
        let slot = Slot::new_with_soda(SlotId::new(1), soda1, 5, 20).unwrap();
        
        assert!(!slot.can_dispense(&soda2));
    }

    #[test]
    fn test_can_dispense_empty_slot() {
        let soda = create_test_soda();
        let slot = Slot::new(SlotId::new(1), 20).unwrap();
        
        assert!(!slot.can_dispense(&soda));
    }

    #[test]
    fn test_can_dispense_disabled_slot() {
        let soda = create_test_soda();
        let mut slot = Slot::new_with_soda(SlotId::new(1), soda.clone(), 5, 20).unwrap();
        slot.disable();
        
        assert!(!slot.can_dispense(&soda));
    }

    #[test]
    fn test_slot_id() {
        let id = SlotId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_slot_id_ordering() {
        let id1 = SlotId::new(1);
        let id2 = SlotId::new(2);
        let id3 = SlotId::new(1);
        
        assert!(id1 < id2);
        assert!(id1 == id3);
        assert!(id2 > id1);
    }

    #[test]
    fn test_display() {
        let soda = create_test_soda();
        let slot = Slot::new_with_soda(SlotId::new(1), soda, 5, 20).unwrap();
        
        assert_eq!(format!("{}", slot), "Slot 1: Coca-Cola (5 of 20) - Enabled");
    }

    #[test]
    fn test_display_empty_slot() {
        let slot = Slot::new(SlotId::new(2), 15).unwrap();
        
        assert_eq!(format!("{}", slot), "Slot 2: Empty (0 of 15) - Enabled");
    }

    #[test]
    fn test_display_disabled_slot() {
        let soda = create_test_soda();
        let mut slot = Slot::new_with_soda(SlotId::new(3), soda, 3, 10).unwrap();
        slot.disable();
        
        assert_eq!(format!("{}", slot), "Slot 3: Coca-Cola (3 of 10) - Disabled");
    }
}
