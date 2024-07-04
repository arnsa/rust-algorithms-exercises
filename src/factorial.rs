pub fn factorial(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }

    return n * factorial(n - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(13), 6227020800);
    }
}
