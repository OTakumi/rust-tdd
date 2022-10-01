// ToDo List
// TODO: $5 + 10 CHF = $10(レートが2:1の場合)
// TODO: Moneyの丸目処理どうする？
// DONE: amountをprivateにする。
// DONE: equals()
// TODO: hashCode()
// TODO: nullとの等価性比較
// TODO: 他オブジェクトとの等価性比較

#[derive(Debug)]
pub struct Doller {
    amount: u32,
}

impl Doller {
    /// Create a Doller instance.
    pub fn new(amount: u32) -> Doller {
        Self { amount }
    }

    /// Calculate multiplication with the entered multiples.
    pub fn times(&mut self, multiplier: u32) -> Doller {
        Doller {
            amount: self.amount * multiplier,
        }
    }

    /// Check the equivalent.
    pub fn equals(&self, other: Doller) -> bool {
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
        assert!(Franc::new(15).equals(five.times(5)));
    }
}
