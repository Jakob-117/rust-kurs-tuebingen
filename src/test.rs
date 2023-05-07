#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::{calculate_bmi, BmiError, Height, Weight};

    #[test]
    fn test_calculate_good() {
        let weight: Weight = Weight::new(69.0);
        let height: Height = Height::new(1.69);
        let result = calculate_bmi(&height, &weight).unwrap();
        assert_float_eq!(result.value(), 24.15, abs <= 0.01);
    }
    #[test]
    fn test_calculate_bad() {
        let weight: Weight = Weight::new(69.0);
        let height: Height = Height::new(0.0);
        let result = calculate_bmi(&height, &weight);
        assert_eq!(result.unwrap_err(), BmiError::HeightIsZero);
    }
    #[test]
    fn test_calculate_bad2() {
        let weight: Weight = Weight::new(69.0);
        let height: Height = Height::new(-1.0);
        let result = calculate_bmi(&height, &weight);
        assert_eq!(result.unwrap_err(), BmiError::HeightIsNegative);
    }
    #[test]
    fn test_calculate_bad3() {
        let weight: Weight = Weight::new(-5.0);
        let height: Height = Height::new(16.0);
        let result = calculate_bmi(&height, &weight);
        assert_eq!(result.unwrap_err(), BmiError::WeightIsNotOk);
    }
    #[test]
    fn test_calculate_bad4() {
        let weight: Weight = Weight::new(0.0);
        let height: Height = Height::new(5.0);
        let result = calculate_bmi(&height, &weight);
        assert_eq!(result.unwrap_err(), BmiError::WeightIsNotOk);
    }
}
