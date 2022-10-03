#[derive(Debug)]
pub struct Money {
    amount: u32,
}

trait MoneyTrait {
    fn new(amount: u32) -> Money;
}

pub struct Doller {}

pub struct Franc {}

impl Money {
    /// Calculate multiplication with the entered multiples.
    pub fn times(&self, multiplier: u32) -> Money {
        Money {
            amount: self.amount * multiplier,
        }
    }
    /// Check the equivalent.
    pub fn equals(&self, target: Money) -> bool {
        self.amount == target.amount
    }

    pub fn doller(amount: u32) -> Money {
        Money { amount }
    }

    pub fn franc(amount: u32) -> Money {
        Money { amount }
    }
}

impl MoneyTrait for Doller {
    /// Create a Doller instance.
    fn new(amount: u32) -> Money {
        Money { amount }
    }
}

impl MoneyTrait for Franc {
    /// Create a Doller instance.
    fn new(amount: u32) -> Money {
        Money { amount }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        // n倍した合計値を確認する
        let five = Money::doller(5);
        assert!(Money::doller(10).equals(five.times(2)));
        assert!(Money::doller(15).equals(five.times(3)));
    }

    #[test]
    fn test_equality() {
        assert!(Money::doller(5).equals(Money::doller(5)));
        assert!(!Money::doller(5).equals(Money::doller(6)));
        assert!(Money::franc(5).equals(Money::franc(5)));
        assert!(!Money::franc(5).equals(Money::franc(6)));
        assert!(!Money::franc(5).equals(Money::doller(5)));
    }

    #[test]
    fn test_franc_multiptication() {
        let five = Franc::new(5);
        assert!(Franc::new(10).equals(five.times(2)));
        assert!(Franc::new(15).equals(five.times(3)));
    }
}
