#[cfg(test)]
mod tests {
    use crate::{calculate_bmi, BmiError, Height, Weight};

    #[test]
    fn test_calculate_good() {
        let weight: Weight = Weight(69.0);
        let height: Height = Height(1.69);
        let result = calculate_bmi(height, weight).unwrap();
        assert_eq!(result.value(), (69.0 / (1.69 * 1.69)));
    }
    #[test]
    fn test_calculate_bad() {
        let weight: Weight = Weight(69.0);
        let height: Height = Height(0.0);
        let result = calculate_bmi(height, weight);
        assert_eq!(result.unwrap_err(), BmiError::HeightIsZero);
    }
    #[test]
    fn test_calculate_bad2() {
        let weight: Weight = Weight(69.0);
        let height: Height = Height(-1.0);
        let result = calculate_bmi(height, weight);
        assert_eq!(result.unwrap_err(), BmiError::HeightIsNegative);
    }
    #[test]
    fn test_calculate_bad3() {
        let weight: Weight = Weight(-5.0);
        let height: Height = Height(16.0);
        let result = calculate_bmi(height, weight);
        assert_eq!(result.unwrap_err(), BmiError::WeightIsNotOk);
    }
    #[test]
    fn test_calculate_bad4() {
        let weight: Weight = Weight(0.0);
        let height: Height = Height(5.0);
        let result = calculate_bmi(height, weight);
        assert_eq!(result.unwrap_err(), BmiError::WeightIsNotOk);
    }
}
