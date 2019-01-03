// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // T1
    // let numbers: Vec<u64> = (1..=num).collect();
    // numbers.iter().product()
    // T2
    // (1..).take_while(|&i| i <= num).product()
    // T3
    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}