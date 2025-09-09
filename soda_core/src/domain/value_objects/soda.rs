use std::fmt;
use super::money::Money;

/// Represents a type of soda with its properties
/// This is a value object that ensures soda operations are consistent
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Soda {
    /// The brand/name of the soda
    name: String,
    /// The flavor of the soda
    flavor: SodaFlavor,
    /// The size of the soda
    size: SodaSize,
    /// The price of the soda
    price: Money,
    /// Whether the soda is diet/sugar-free
    is_diet: bool,
    /// Whether the soda is caffeinated
    is_caffeinated: bool,
}

/// Available soda flavors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SodaFlavor {
    Cola,
    Orange,
    LemonLime,
    RootBeer,
    Grape,
    Cherry,
    Vanilla,
    Strawberry,
    Peach,
    Watermelon,
}

/// Available soda sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SodaSize {
    Small,   // 8 oz
    Medium,  // 12 oz
    Large,   // 16 oz
    XLarge,  // 20 oz
}

/// Errors that can occur during soda operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SodaError {
    InvalidName(String),
    InvalidPrice,
    InvalidSize,
}

impl Soda {
    /// Creates a new Soda instance
    /// 
    /// # Arguments
    /// * `name` - The brand/name of the soda
    /// * `flavor` - The flavor of the soda
    /// * `size` - The size of the soda
    /// * `price` - The price of the soda
    /// * `is_diet` - Whether the soda is diet/sugar-free
    /// * `is_caffeinated` - Whether the soda is caffeinated
    /// 
    /// # Returns
    /// * `Result<Soda, SodaError>` - Ok(Soda) if valid, Err if invalid
    /// 
    /// # Examples
    /// ```
    /// use soda_core::domain::value_objects::soda::{Soda, SodaFlavor, SodaSize};
    /// use soda_core::domain::value_objects::money::Money;
    /// 
    /// let soda = Soda::new(
    ///     "Coca-Cola".to_string(),
    ///     SodaFlavor::Cola,
    ///     SodaSize::Medium,
    ///     Money::from_dollars_cents(1, 50).unwrap(),
    ///     false,
    ///     true
    /// ).unwrap();
    /// ```
    pub fn new(
        name: String,
        flavor: SodaFlavor,
        size: SodaSize,
        price: Money,
        is_diet: bool,
        is_caffeinated: bool,
    ) -> Result<Self, SodaError> {
        if name.trim().is_empty() {
            return Err(SodaError::InvalidName("Soda name cannot be empty".to_string()));
        }

        if price.is_negative() {
            return Err(SodaError::InvalidPrice);
        }

        Ok(Soda {
            name: name.trim().to_string(),
            flavor,
            size,
            price,
            is_diet,
            is_caffeinated,
        })
    }

    /// Gets the name of the soda
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the flavor of the soda
    pub fn flavor(&self) -> SodaFlavor {
        self.flavor
    }

    /// Gets the size of the soda
    pub fn size(&self) -> SodaSize {
        self.size
    }

    /// Gets the price of the soda
    pub fn price(&self) -> Money {
        self.price
    }

    /// Checks if the soda is diet/sugar-free
    pub fn is_diet(&self) -> bool {
        self.is_diet
    }

    /// Checks if the soda is caffeinated
    pub fn is_caffeinated(&self) -> bool {
        self.is_caffeinated
    }

    /// Gets the volume in ounces for the soda size
    pub fn volume_ounces(&self) -> u8 {
        match self.size {
            SodaSize::Small => 8,
            SodaSize::Medium => 12,
            SodaSize::Large => 16,
            SodaSize::XLarge => 20,
        }
    }

    /// Creates a new soda with a different size and adjusted price
    /// 
    /// # Arguments
    /// * `new_size` - The new size for the soda
    /// * `size_multiplier` - The price multiplier for the new size
    /// 
    /// # Returns
    /// * `Result<Soda, SodaError>` - Ok(Soda) with new size and price, Err if invalid
    pub fn with_size(self, new_size: SodaSize, size_multiplier: f64) -> Result<Self, SodaError> {
        if size_multiplier <= 0.0 || size_multiplier.is_nan() || size_multiplier.is_infinite() {
            return Err(SodaError::InvalidPrice);
        }

        let new_price = Money::from_decimal(self.price.as_decimal() * size_multiplier)
            .map_err(|_| SodaError::InvalidPrice)?;

        Ok(Soda {
            name: self.name,
            flavor: self.flavor,
            size: new_size,
            price: new_price,
            is_diet: self.is_diet,
            is_caffeinated: self.is_caffeinated,
        })
    }

    /// Creates a new soda with a different price
    /// 
    /// # Arguments
    /// * `new_price` - The new price for the soda
    /// 
    /// # Returns
    /// * `Result<Soda, SodaError>` - Ok(Soda) with new price, Err if invalid
    pub fn with_price(self, new_price: Money) -> Result<Self, SodaError> {
        if new_price.is_negative() {
            return Err(SodaError::InvalidPrice);
        }

        Ok(Soda {
            name: self.name,
            flavor: self.flavor,
            size: self.size,
            price: new_price,
            is_diet: self.is_diet,
            is_caffeinated: self.is_caffeinated,
        })
    }

    /// Checks if this soda is the same type as another (ignoring size and price)
    pub fn is_same_type(&self, other: &Soda) -> bool {
        self.name == other.name && self.flavor == other.flavor
    }

    /// Gets a description of the soda
    pub fn description(&self) -> String {
        let diet_text = if self.is_diet { "Diet " } else { "" };
        let caffeine_text = if self.is_caffeinated { " (Caffeinated)" } else { " (Caffeine-free)" };
        format!(
            "{}{} {} - {} oz{}",
            diet_text,
            self.name,
            self.flavor.to_string(),
            self.volume_ounces(),
            caffeine_text
        )
    }
}

impl SodaFlavor {
    /// Gets a human-readable string representation of the flavor
    pub fn to_string(&self) -> String {
        match self {
            SodaFlavor::Cola => "Cola".to_string(),
            SodaFlavor::Orange => "Orange".to_string(),
            SodaFlavor::LemonLime => "Lemon-Lime".to_string(),
            SodaFlavor::RootBeer => "Root Beer".to_string(),
            SodaFlavor::Grape => "Grape".to_string(),
            SodaFlavor::Cherry => "Cherry".to_string(),
            SodaFlavor::Vanilla => "Vanilla".to_string(),
            SodaFlavor::Strawberry => "Strawberry".to_string(),
            SodaFlavor::Peach => "Peach".to_string(),
            SodaFlavor::Watermelon => "Watermelon".to_string(),
        }
    }

    /// Gets the flavor from a string representation
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "cola" => Some(SodaFlavor::Cola),
            "orange" => Some(SodaFlavor::Orange),
            "lemon-lime" | "lemonlime" => Some(SodaFlavor::LemonLime),
            "root beer" | "rootbeer" => Some(SodaFlavor::RootBeer),
            "grape" => Some(SodaFlavor::Grape),
            "cherry" => Some(SodaFlavor::Cherry),
            "vanilla" => Some(SodaFlavor::Vanilla),
            "strawberry" => Some(SodaFlavor::Strawberry),
            "peach" => Some(SodaFlavor::Peach),
            "watermelon" => Some(SodaFlavor::Watermelon),
            _ => None,
        }
    }
}

impl SodaSize {
    /// Gets a human-readable string representation of the size
    pub fn to_string(&self) -> String {
        match self {
            SodaSize::Small => "Small (8 oz)".to_string(),
            SodaSize::Medium => "Medium (12 oz)".to_string(),
            SodaSize::Large => "Large (16 oz)".to_string(),
            SodaSize::XLarge => "X-Large (20 oz)".to_string(),
        }
    }

    /// Gets the size from a string representation
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "small" | "s" => Some(SodaSize::Small),
            "medium" | "m" => Some(SodaSize::Medium),
            "large" | "l" => Some(SodaSize::Large),
            "x-large" | "xlarge" | "xl" => Some(SodaSize::XLarge),
            _ => None,
        }
    }

    /// Gets the volume in ounces for the size
    pub fn volume_ounces(&self) -> u8 {
        match self {
            SodaSize::Small => 8,
            SodaSize::Medium => 12,
            SodaSize::Large => 16,
            SodaSize::XLarge => 20,
        }
    }
}

impl fmt::Display for Soda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl fmt::Display for SodaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SodaError::InvalidName(msg) => write!(f, "Invalid name: {}", msg),
            SodaError::InvalidPrice => write!(f, "Invalid price"),
            SodaError::InvalidSize => write!(f, "Invalid size"),
        }
    }
}

impl std::error::Error for SodaError {}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_soda_creation() {
        let soda = create_test_soda();
        
        assert_eq!(soda.name(), "Coca-Cola");
        assert_eq!(soda.flavor(), SodaFlavor::Cola);
        assert_eq!(soda.size(), SodaSize::Medium);
        assert_eq!(soda.price(), Money::from_dollars_cents(1, 50).unwrap());
        assert_eq!(soda.is_diet(), false);
        assert_eq!(soda.is_caffeinated(), true);
        assert_eq!(soda.volume_ounces(), 12);
    }

    #[test]
    fn test_soda_creation_empty_name() {
        let result = Soda::new(
            "".to_string(),
            SodaFlavor::Cola,
            SodaSize::Medium,
            Money::from_dollars_cents(1, 50).unwrap(),
            false,
            true,
        );
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaError::InvalidName("Soda name cannot be empty".to_string()));
    }

    #[test]
    fn test_soda_creation_whitespace_name() {
        let result = Soda::new(
            "   ".to_string(),
            SodaFlavor::Cola,
            SodaSize::Medium,
            Money::from_dollars_cents(1, 50).unwrap(),
            false,
            true,
        );
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaError::InvalidName("Soda name cannot be empty".to_string()));
    }

    #[test]
    fn test_soda_creation_negative_price() {
        let result = Soda::new(
            "Coca-Cola".to_string(),
            SodaFlavor::Cola,
            SodaSize::Medium,
            Money::from_dollars_cents(-1, 50).unwrap(),
            false,
            true,
        );
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaError::InvalidPrice);
    }

    #[test]
    fn test_soda_creation_trimmed_name() {
        let soda = Soda::new(
            "  Coca-Cola  ".to_string(),
            SodaFlavor::Cola,
            SodaSize::Medium,
            Money::from_dollars_cents(1, 50).unwrap(),
            false,
            true,
        ).unwrap();
        
        assert_eq!(soda.name(), "Coca-Cola");
    }

    #[test]
    fn test_volume_ounces() {
        assert_eq!(SodaSize::Small.volume_ounces(), 8);
        assert_eq!(SodaSize::Medium.volume_ounces(), 12);
        assert_eq!(SodaSize::Large.volume_ounces(), 16);
        assert_eq!(SodaSize::XLarge.volume_ounces(), 20);
    }

    #[test]
    fn test_with_size() {
        let soda = create_test_soda();
        let large_soda = soda.with_size(SodaSize::Large, 1.5).unwrap();
        
        assert_eq!(large_soda.size(), SodaSize::Large);
        assert_eq!(large_soda.price(), Money::from_dollars_cents(2, 25).unwrap()); // $1.50 * 1.5
        assert_eq!(large_soda.volume_ounces(), 16);
    }

    #[test]
    fn test_with_size_invalid_multiplier() {
        let soda = create_test_soda();
        let result = soda.with_size(SodaSize::Large, -1.0);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaError::InvalidPrice);
    }

    #[test]
    fn test_with_price() {
        let soda = create_test_soda();
        let new_price = Money::from_dollars_cents(2, 00).unwrap();
        let updated_soda = soda.with_price(new_price).unwrap();
        
        assert_eq!(updated_soda.price(), new_price);
        assert_eq!(updated_soda.name(), "Coca-Cola"); // Other properties unchanged
    }

    #[test]
    fn test_with_price_negative() {
        let soda = create_test_soda();
        let negative_price = Money::from_dollars_cents(-1, 00).unwrap();
        let result = soda.with_price(negative_price);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SodaError::InvalidPrice);
    }

    #[test]
    fn test_is_same_type() {
        let soda1 = Soda::new(
            "Coca-Cola".to_string(),
            SodaFlavor::Cola,
            SodaSize::Medium,
            Money::from_dollars_cents(1, 50).unwrap(),
            false,
            true,
        ).unwrap();

        let soda2 = Soda::new(
            "Coca-Cola".to_string(),
            SodaFlavor::Cola,
            SodaSize::Large, // Different size
            Money::from_dollars_cents(2, 00).unwrap(), // Different price
            true, // Different diet status
            false, // Different caffeine status
        ).unwrap();

        assert!(soda1.is_same_type(&soda2));
    }

    #[test]
    fn test_is_same_type_different() {
        let soda1 = Soda::new(
            "Coca-Cola".to_string(),
            SodaFlavor::Cola,
            SodaSize::Medium,
            Money::from_dollars_cents(1, 50).unwrap(),
            false,
            true,
        ).unwrap();

        let soda2 = Soda::new(
            "Pepsi".to_string(), // Different name
            SodaFlavor::Cola,
            SodaSize::Medium,
            Money::from_dollars_cents(1, 50).unwrap(),
            false,
            true,
        ).unwrap();

        assert!(!soda1.is_same_type(&soda2));
    }

    #[test]
    fn test_description() {
        let soda = Soda::new(
            "Coca-Cola".to_string(),
            SodaFlavor::Cola,
            SodaSize::Medium,
            Money::from_dollars_cents(1, 50).unwrap(),
            false,
            true,
        ).unwrap();

        assert_eq!(soda.description(), "Coca-Cola Cola - 12 oz (Caffeinated)");
    }

    #[test]
    fn test_description_diet() {
        let soda = Soda::new(
            "Diet Coke".to_string(),
            SodaFlavor::Cola,
            SodaSize::Large,
            Money::from_dollars_cents(1, 75).unwrap(),
            true,
            true,
        ).unwrap();

        assert_eq!(soda.description(), "Diet Diet Coke Cola - 16 oz (Caffeinated)");
    }

    #[test]
    fn test_description_caffeine_free() {
        let soda = Soda::new(
            "Sprite".to_string(),
            SodaFlavor::LemonLime,
            SodaSize::Small,
            Money::from_dollars_cents(1, 25).unwrap(),
            false,
            false,
        ).unwrap();

        assert_eq!(soda.description(), "Sprite Lemon-Lime - 8 oz (Caffeine-free)");
    }

    #[test]
    fn test_flavor_to_string() {
        assert_eq!(SodaFlavor::Cola.to_string(), "Cola");
        assert_eq!(SodaFlavor::LemonLime.to_string(), "Lemon-Lime");
        assert_eq!(SodaFlavor::RootBeer.to_string(), "Root Beer");
    }

    #[test]
    fn test_flavor_from_string() {
        assert_eq!(SodaFlavor::from_string("cola"), Some(SodaFlavor::Cola));
        assert_eq!(SodaFlavor::from_string("lemon-lime"), Some(SodaFlavor::LemonLime));
        assert_eq!(SodaFlavor::from_string("root beer"), Some(SodaFlavor::RootBeer));
        assert_eq!(SodaFlavor::from_string("invalid"), None);
    }

    #[test]
    fn test_size_to_string() {
        assert_eq!(SodaSize::Small.to_string(), "Small (8 oz)");
        assert_eq!(SodaSize::Medium.to_string(), "Medium (12 oz)");
        assert_eq!(SodaSize::Large.to_string(), "Large (16 oz)");
        assert_eq!(SodaSize::XLarge.to_string(), "X-Large (20 oz)");
    }

    #[test]
    fn test_size_from_string() {
        assert_eq!(SodaSize::from_string("small"), Some(SodaSize::Small));
        assert_eq!(SodaSize::from_string("s"), Some(SodaSize::Small));
        assert_eq!(SodaSize::from_string("medium"), Some(SodaSize::Medium));
        assert_eq!(SodaSize::from_string("m"), Some(SodaSize::Medium));
        assert_eq!(SodaSize::from_string("large"), Some(SodaSize::Large));
        assert_eq!(SodaSize::from_string("l"), Some(SodaSize::Large));
        assert_eq!(SodaSize::from_string("x-large"), Some(SodaSize::XLarge));
        assert_eq!(SodaSize::from_string("xl"), Some(SodaSize::XLarge));
        assert_eq!(SodaSize::from_string("invalid"), None);
    }

    #[test]
    fn test_size_ordering() {
        assert!(SodaSize::Small < SodaSize::Medium);
        assert!(SodaSize::Medium < SodaSize::Large);
        assert!(SodaSize::Large < SodaSize::XLarge);
    }

    #[test]
    fn test_display() {
        let soda = create_test_soda();
        assert_eq!(format!("{}", soda), "Coca-Cola Cola - 12 oz (Caffeinated)");
    }
}
