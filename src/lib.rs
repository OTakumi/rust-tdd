// ToDo List
// TODO: $5 + 10 CHF = $10
// TODO: Moneyの丸目処理どうする？
// DONE: equals()
// TODO: hashCode()

#[derive(Debug)]
pub struct Doller {
    pub amount: u32,
}

impl Doller {
    /// Create a Doller instance.
    pub fn new(amount: u32) -> Doller {
        Doller { amount }
    }

    /// Calculate multiplication with the entered multiples.
    pub fn times(&mut self, multiplier: u32) -> Doller {
        Doller {
            amount: self.amount * multiplier,
        }
    }

    /// Check the equivalent.
    pub fn equals(&self, other: &Doller) -> bool {
        self.amount == other.amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        // n倍した合計値を確認する
        let mut five = Doller::new(5);
        let mut product = five.times(2);
        assert_eq!(10, product.amount);

        product = five.times(3);
        assert_eq!(15, product.amount);
    }

    #[test]
    fn test_equality() {
        assert!(Doller::new(5).equals(&Doller::new(5)));
        assert!(Doller::new(5).equals(&Doller::new(6)));
    }
}
