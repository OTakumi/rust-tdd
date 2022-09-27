// ToDo List
// TODO: $5 + 10 CHF = $10
// TODO: Moneyの丸目処理どうする？
// TODO: equals()
// TODO: hashCode()

#[derive(Debug)]
pub struct Doller {
    pub amount: u32,
}

impl Doller {
    pub fn new(amount: u32) -> Doller {
        Doller { amount }
    }

    pub fn times(&mut self, multiplier: u32) -> Doller {
        Doller {
            amount: self.amount * multiplier,
        }
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
}
