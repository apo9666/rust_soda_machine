use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

/// Represents a monetary amount with currency and precision
/// This is a value object that ensures money operations are safe and consistent
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Money {
    /// Amount in cents to avoid floating point precision issues
    cents: i64,
}

/// Currency types supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
}

/// Errors that can occur during money operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoneyError {
    InvalidAmount(String),
    CurrencyMismatch,
    DivisionByZero,
    Overflow,
    Underflow,
}

impl Money {
    /// Creates a new Money instance from dollars and cents
    /// 
    /// # Arguments
    /// * `dollars` - The dollar amount (can be negative)
    /// * `cents` - The cents amount (0-99)
    /// 
    /// # Returns
    /// * `Result<Money, MoneyError>` - Ok(Money) if valid, Err if invalid
    /// 
    /// # Examples
    /// ```
    /// use soda_core::domain::value_objects::money::Money;
    /// 
    /// let money = Money::from_dollars_cents(5, 25).unwrap(); // $5.25
    /// let negative = Money::from_dollars_cents(-2, 50).unwrap(); // -$2.50
    /// ```
    pub fn from_dollars_cents(dollars: i64, cents: u8) -> Result<Self, MoneyError> {
        if cents > 99 {
            return Err(MoneyError::InvalidAmount("Cents must be between 0 and 99".to_string()));
        }

        let total_cents = match dollars.checked_mul(100) {
            Some(dollars_cents) => {
                if dollars >= 0 {
                    match dollars_cents.checked_add(cents as i64) {
                        Some(total) => total,
                        None => return Err(MoneyError::Overflow),
                    }
                } else {
                    match dollars_cents.checked_sub(cents as i64) {
                        Some(total) => total,
                        None => return Err(MoneyError::Underflow),
                    }
                }
            },
            None => return Err(MoneyError::Overflow),
        };

        Ok(Money { cents: total_cents })
    }

    /// Creates a new Money instance from a decimal amount
    /// 
    /// # Arguments
    /// * `amount` - The decimal amount (e.g., 5.25 for $5.25)
    /// 
    /// # Returns
    /// * `Result<Money, MoneyError>` - Ok(Money) if valid, Err if invalid
    pub fn from_decimal(amount: f64) -> Result<Self, MoneyError> {
        if amount.is_nan() || amount.is_infinite() {
            return Err(MoneyError::InvalidAmount("Amount cannot be NaN or infinite".to_string()));
        }

        // Round to nearest cent to avoid floating point precision issues
        let rounded = (amount * 100.0).round() as i64;
        
        Ok(Money { cents: rounded })
    }

    /// Creates a new Money instance from cents
    /// 
    /// # Arguments
    /// * `cents` - The amount in cents
    /// 
    /// # Returns
    /// * `Money` - The money instance
    pub fn from_cents(cents: i64) -> Self {
        Money { cents }
    }

    /// Creates a zero amount
    pub fn zero() -> Self {
        Money { cents: 0 }
    }

    /// Gets the total amount in cents
    pub fn cents(&self) -> i64 {
        self.cents
    }

    /// Gets the dollar portion of the amount
    pub fn dollars(&self) -> i64 {
        self.cents / 100
    }

    /// Gets the cents portion of the amount (0-99)
    pub fn cents_portion(&self) -> u8 {
        (self.cents.abs() % 100) as u8
    }

    /// Gets the amount as a decimal (e.g., 5.25 for $5.25)
    pub fn as_decimal(&self) -> f64 {
        self.cents as f64 / 100.0
    }

    /// Checks if the amount is zero
    pub fn is_zero(&self) -> bool {
        self.cents == 0
    }

    /// Checks if the amount is positive
    pub fn is_positive(&self) -> bool {
        self.cents > 0
    }

    /// Checks if the amount is negative
    pub fn is_negative(&self) -> bool {
        self.cents < 0
    }

    /// Returns the absolute value of the money
    pub fn abs(self) -> Self {
        Money { cents: self.cents.abs() }
    }

    /// Returns the negative of the money
    pub fn neg(self) -> Self {
        Money { cents: -self.cents }
    }
}

impl Add for Money {
    type Output = Result<Money, MoneyError>;

    fn add(self, other: Money) -> Self::Output {
        match self.cents.checked_add(other.cents) {
            Some(cents) => Ok(Money { cents }),
            None => Err(MoneyError::Overflow),
        }
    }
}

impl Sub for Money {
    type Output = Result<Money, MoneyError>;

    fn sub(self, other: Money) -> Self::Output {
        match self.cents.checked_sub(other.cents) {
            Some(cents) => Ok(Money { cents }),
            None => Err(MoneyError::Underflow),
        }
    }
}

impl Mul<i64> for Money {
    type Output = Result<Money, MoneyError>;

    fn mul(self, multiplier: i64) -> Self::Output {
        match self.cents.checked_mul(multiplier) {
            Some(cents) => Ok(Money { cents }),
            None => Err(MoneyError::Overflow),
        }
    }
}

impl Div<i64> for Money {
    type Output = Result<Money, MoneyError>;

    fn div(self, divisor: i64) -> Self::Output {
        if divisor == 0 {
            return Err(MoneyError::DivisionByZero);
        }
        
        Ok(Money { cents: self.cents / divisor })
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dollars = self.dollars();
        let cents = self.cents_portion();
        
        if self.is_negative() {
            write!(f, "-${}.{:02}", dollars.abs(), cents)
        } else {
            write!(f, "${}.{:02}", dollars, cents)
        }
    }
}

impl fmt::Display for MoneyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoneyError::InvalidAmount(msg) => write!(f, "Invalid amount: {}", msg),
            MoneyError::CurrencyMismatch => write!(f, "Currency mismatch"),
            MoneyError::DivisionByZero => write!(f, "Division by zero"),
            MoneyError::Overflow => write!(f, "Arithmetic overflow"),
            MoneyError::Underflow => write!(f, "Arithmetic underflow"),
        }
    }
}

impl std::error::Error for MoneyError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_dollars_cents() {
        let money = Money::from_dollars_cents(5, 25).unwrap();
        assert_eq!(money.cents(), 525);
        assert_eq!(money.dollars(), 5);
        assert_eq!(money.cents_portion(), 25);
        assert_eq!(money.as_decimal(), 5.25);
    }

    #[test]
    fn test_from_dollars_cents_negative() {
        let money = Money::from_dollars_cents(-2, 50).unwrap();
        assert_eq!(money.cents(), -250);
        assert_eq!(money.dollars(), -2);
        assert_eq!(money.cents_portion(), 50);
        assert_eq!(money.as_decimal(), -2.50);
    }

    #[test]
    fn test_from_dollars_cents_invalid_cents() {
        let result = Money::from_dollars_cents(5, 100);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoneyError::InvalidAmount("Cents must be between 0 and 99".to_string()));
    }

    #[test]
    fn test_from_decimal() {
        let money = Money::from_decimal(5.25).unwrap();
        assert_eq!(money.cents(), 525);
        assert_eq!(money.as_decimal(), 5.25);
    }

    #[test]
    fn test_from_decimal_negative() {
        let money = Money::from_decimal(-3.75).unwrap();
        assert_eq!(money.cents(), -375);
        assert_eq!(money.as_decimal(), -3.75);
    }

    #[test]
    fn test_from_cents() {
        let money = Money::from_cents(1234);
        assert_eq!(money.cents(), 1234);
        assert_eq!(money.dollars(), 12);
        assert_eq!(money.cents_portion(), 34);
    }

    #[test]
    fn test_zero() {
        let money = Money::zero();
        assert_eq!(money.cents(), 0);
        assert!(money.is_zero());
        assert!(!money.is_positive());
        assert!(!money.is_negative());
    }

    #[test]
    fn test_is_positive_negative() {
        let positive = Money::from_cents(100);
        let negative = Money::from_cents(-100);
        let zero = Money::zero();

        assert!(positive.is_positive());
        assert!(!positive.is_negative());
        assert!(!positive.is_zero());

        assert!(!negative.is_positive());
        assert!(negative.is_negative());
        assert!(!negative.is_zero());

        assert!(!zero.is_positive());
        assert!(!zero.is_negative());
        assert!(zero.is_zero());
    }

    #[test]
    fn test_abs() {
        let positive = Money::from_cents(100);
        let negative = Money::from_cents(-100);

        assert_eq!(positive.abs().cents(), 100);
        assert_eq!(negative.abs().cents(), 100);
    }

    #[test]
    fn test_neg() {
        let positive = Money::from_cents(100);
        let negative = Money::from_cents(-100);

        assert_eq!(positive.neg().cents(), -100);
        assert_eq!(negative.neg().cents(), 100);
    }

    #[test]
    fn test_add() {
        let money1 = Money::from_cents(100);
        let money2 = Money::from_cents(250);
        let result = (money1 + money2).unwrap();
        
        assert_eq!(result.cents(), 350);
    }

    #[test]
    fn test_add_negative() {
        let money1 = Money::from_cents(100);
        let money2 = Money::from_cents(-50);
        let result = (money1 + money2).unwrap();
        
        assert_eq!(result.cents(), 50);
    }

    #[test]
    fn test_sub() {
        let money1 = Money::from_cents(350);
        let money2 = Money::from_cents(100);
        let result = (money1 - money2).unwrap();
        
        assert_eq!(result.cents(), 250);
    }

    #[test]
    fn test_mul() {
        let money = Money::from_cents(100);
        let result = (money * 3).unwrap();
        
        assert_eq!(result.cents(), 300);
    }

    #[test]
    fn test_div() {
        let money = Money::from_cents(300);
        let result = (money / 3).unwrap();
        
        assert_eq!(result.cents(), 100);
    }

    #[test]
    fn test_div_by_zero() {
        let money = Money::from_cents(100);
        let result = money / 0;
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), MoneyError::DivisionByZero);
    }

    #[test]
    fn test_display() {
        let positive = Money::from_cents(525);
        let negative = Money::from_cents(-250);
        let zero = Money::zero();

        assert_eq!(format!("{}", positive), "$5.25");
        assert_eq!(format!("{}", negative), "-$2.50");
        assert_eq!(format!("{}", zero), "$0.00");
    }

    #[test]
    fn test_ordering() {
        let money1 = Money::from_cents(100);
        let money2 = Money::from_cents(200);
        let money3 = Money::from_cents(100);

        assert!(money1 < money2);
        assert!(money2 > money1);
        assert_eq!(money1, money3);
    }
}
