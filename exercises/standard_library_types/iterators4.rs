// iterators4.rs


pub fn factorial(num: u64) -> u64 {
    /* additional variables + unrecursion + for */
    // let mut a: u64 = 1;
    // for i in 1..=num {
    //     a *= i; 
    // }
    // a

    /* return and factorial */
    // if num == 1 {return 1;};
    // num * factorial(num-1)

    /* fatorial */
    // if num != 1 {
    //     num * factorial(num-1)
    // } else {
    //     num 
    // }

    // std::iter::Iterator::fold
    // (1..num+1).fold(1, |sum, v| sum * v)

    // std::iter::Iterator::reduce
    (1..=num).reduce(|sum, v| sum * v).unwrap()


    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
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
