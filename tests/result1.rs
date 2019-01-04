// result1.rs

use std::cmp::Ordering;

#[derive(PartialEq,Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq,Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        /*
        if value == 0 {
            Err(CreationError::Zero)
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }*/
        match value.cmp(&0) {
            Ordering::Less => Err(CreationError::Negative),
            Ordering::Greater => Ok(PositiveNonzeroInteger(value as u64)),
            Ordering::Equal => Err(CreationError::Zero),
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(Err(CreationError::Negative), PositiveNonzeroInteger::new(-10));
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}