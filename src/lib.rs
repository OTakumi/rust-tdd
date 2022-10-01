#[derive(Debug)]
pub struct Money {
    amount: u32,
}

trait MoneyTrait {
    fn new(amount: u32) -> Money;
}

pub struct Doller {}

struct Franc {
    amount: u32,
}

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
}

impl MoneyTrait for Doller {
    /// Create a Doller instance.
    fn new(amount: u32) -> Money {
        Money { amount }
    }
}

impl Franc {
    /// Create a Doller instance.
    pub fn new(amount: u32) -> Franc {
        Franc { amount }
    }

    /// Calculate multiplication with the entered multiples.
    pub fn times(&mut self, multiplier: u32) -> Franc {
        Franc {
            amount: self.amount * multiplier,
        }
    }

    /// Check the equivalent.
    pub fn equals(&self, other: Franc) -> bool {
        self.amount == other.amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        // n倍した合計値を確認する
        let five = Doller::new(5);
        assert!(Doller::new(10).equals(five.times(2)));
        assert!(Doller::new(15).equals(five.times(3)));
    }

    #[test]
    fn test_equality() {
        assert!(Doller::new(5).equals(Doller::new(5)));
        assert!(!Doller::new(5).equals(Doller::new(6)));
    }

    #[test]
    fn test_franc_multiptication() {
        let mut five = Franc::new(5);
        assert!(Franc::new(10).equals(five.times(2)));
        assert!(Franc::new(15).equals(five.times(3)));
    }
}
