pub fn results_per_page(min: i64, max: i64, requested: i64) -> i64 {
    if requested > max {
        return max;
    }

    if requested < min {
        return min
    }

    requested
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn results_count_is_good() {
        let result = results_per_page(10, 100, 55);
        assert_eq!(result, 55);
    }

    #[test]
    fn results_count_is_low() {
        let result = results_per_page(10, 100, 3);
        assert_eq!(result, 10);
    }

    #[test]
    fn results_count_is_high() {
        let result = results_per_page(10, 100, 199);
        assert_eq!(result, 100);
    }
}
