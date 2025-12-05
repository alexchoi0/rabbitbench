use crate::db::models::Threshold;

pub fn check_threshold(
    threshold: &Threshold,
    new_value: f64,
    baseline_values: &[f64],
) -> Option<ThresholdViolation> {
    if baseline_values.len() < threshold.min_sample_size as usize {
        return None;
    }

    let baseline_avg: f64 = baseline_values.iter().sum::<f64>() / baseline_values.len() as f64;

    if baseline_avg == 0.0 {
        return None;
    }

    let percent_change = ((new_value - baseline_avg) / baseline_avg) * 100.0;

    if let Some(upper) = threshold.upper_boundary {
        if percent_change > upper {
            return Some(ThresholdViolation {
                baseline_value: baseline_avg,
                percent_change,
                violation_type: ViolationType::Upper,
            });
        }
    }

    if let Some(lower) = threshold.lower_boundary {
        if percent_change < -lower {
            return Some(ThresholdViolation {
                baseline_value: baseline_avg,
                percent_change,
                violation_type: ViolationType::Lower,
            });
        }
    }

    None
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ThresholdViolation {
    pub baseline_value: f64,
    pub percent_change: f64,
    pub violation_type: ViolationType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViolationType {
    Upper,
    Lower,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_threshold(
        upper: Option<f64>,
        lower: Option<f64>,
        min_samples: i32,
    ) -> Threshold {
        Threshold {
            id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            branch_id: None,
            testbed_id: None,
            measure_id: Uuid::new_v4(),
            upper_boundary: upper,
            lower_boundary: lower,
            min_sample_size: min_samples,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_no_violation_within_bounds() {
        let threshold = create_test_threshold(Some(10.0), Some(10.0), 2);
        let baseline = vec![100.0, 100.0];

        let result = check_threshold(&threshold, 105.0, &baseline);
        assert!(result.is_none());
    }

    #[test]
    fn test_upper_violation() {
        let threshold = create_test_threshold(Some(10.0), None, 2);
        let baseline = vec![100.0, 100.0];

        let result = check_threshold(&threshold, 115.0, &baseline);
        assert!(result.is_some());
        let violation = result.unwrap();
        assert_eq!(violation.violation_type, ViolationType::Upper);
        assert!((violation.percent_change - 15.0).abs() < 0.01);
    }

    #[test]
    fn test_insufficient_samples() {
        let threshold = create_test_threshold(Some(10.0), None, 5);
        let baseline = vec![100.0, 100.0];

        let result = check_threshold(&threshold, 200.0, &baseline);
        assert!(result.is_none());
    }
}
