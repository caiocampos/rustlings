// iterator3.rs

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// This function should calculate `a` divided by `b` if `a` is
// evenly divisible by b.
// Otherwise, it should return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    /*
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a % b == 0 {
        Ok(a / b)
    } else {
        Err(DivisionError::NotDivisible(NotDivisibleError {dividend: a, divisor: b}))
    }*/
    match a.checked_rem(b) {
        None => Err(DivisionError::DivideByZero),
        Some(0) => Ok(a / b),
        Some(_) => Err(DivisionError::NotDivisible(NotDivisibleError {dividend: a, divisor: b})),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests that verify your `divide` function implementation
    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError{
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    // Iterator exercises using your `divide` function
    #[test]
    fn result_with_list() {
        let numbers = vec![27, 297, 38502, 81];
        let division_results = numbers.into_iter().map(|n| divide(n, 27));
        // let x: Result<Vec<_>, _> = division_results.collect();
        let x = division_results.collect::<Result<Vec<_>, _>>();
        assert_eq!(format!("{:?}", x), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn list_of_results() {
        let numbers = vec![27, 297, 38502, 81];
        let division_results = numbers.into_iter().map(|n| divide(n, 27));
        // let x: Vec<_> = division_results.collect();
        let x = division_results.collect::<Vec<_>>();
        assert_eq!(format!("{:?}", x), "[Ok(1), Ok(11), Ok(1426), Ok(3)]");
    }
}