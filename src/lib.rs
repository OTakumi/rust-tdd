#[derive(Debug)]
pub struct Money {
    amount: u32,
    currency: &'static str,
}

trait MoneyTrait {
    fn new(amount: u32) -> Money;
}

impl Money {
    /// Calculate multiplication with the entered multiples.
    pub fn times(&self, multiplier: u32) -> Money {
        Money {
            amount: self.amount * multiplier,
            currency: &self.currency,
        }
    }
    /// Check the equivalent.
    pub fn equals(&self, target: Money) -> bool {
        self.amount == target.amount
    }

    pub fn doller(amount: u32) -> Money {
        Money {
            amount,
            currency: "USD",
        }
    }

    pub fn franc(amount: u32) -> Money {
        Money {
            amount,
            currency: "CHF",
        }
    }

    pub fn currency(&self) -> &'static str {
        self.currency
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
        let five = Money::franc(5);
        assert!(Money::franc(10).equals(five.times(2)));
        assert!(Money::franc(15).equals(five.times(3)));
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::doller(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }
}
