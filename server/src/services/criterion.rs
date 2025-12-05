use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriterionResult {
    pub name: String,
    pub value: f64,
    pub lower: Option<f64>,
    pub upper: Option<f64>,
}

#[allow(dead_code)]
static CRITERION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?m)^(\S+)\s+time:\s+\[([0-9.]+)\s+(ns|µs|ms|s)\s+([0-9.]+)\s+(ns|µs|ms|s)\s+([0-9.]+)\s+(ns|µs|ms|s)\]"
    ).unwrap()
});

#[allow(dead_code)]
pub fn parse_criterion_output(output: &str) -> Vec<CriterionResult> {
    CRITERION_REGEX
        .captures_iter(output)
        .filter_map(|cap| {
            let name = cap.get(1)?.as_str().to_string();
            let lower = parse_time(cap.get(2)?.as_str(), cap.get(3)?.as_str())?;
            let mean = parse_time(cap.get(4)?.as_str(), cap.get(5)?.as_str())?;
            let upper = parse_time(cap.get(6)?.as_str(), cap.get(7)?.as_str())?;

            Some(CriterionResult {
                name,
                value: mean,
                lower: Some(lower),
                upper: Some(upper),
            })
        })
        .collect()
}

#[allow(dead_code)]
fn parse_time(value: &str, unit: &str) -> Option<f64> {
    let v: f64 = value.parse().ok()?;
    let multiplier = match unit {
        "ns" => 1.0,
        "µs" | "us" => 1_000.0,
        "ms" => 1_000_000.0,
        "s" => 1_000_000_000.0,
        _ => return None,
    };
    Some(v * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_criterion_output() {
        let output = r#"
Benchmarking fibonacci/10
fibonacci/10            time:   [1.2345 µs 1.2456 µs 1.2567 µs]

Benchmarking fibonacci/20
fibonacci/20            time:   [123.45 ns 124.56 ns 125.67 ns]
        "#;

        let results = parse_criterion_output(output);
        assert_eq!(results.len(), 2);

        assert_eq!(results[0].name, "fibonacci/10");
        assert!((results[0].value - 1245.6).abs() < 0.1);

        assert_eq!(results[1].name, "fibonacci/20");
        assert!((results[1].value - 124.56).abs() < 0.01);
    }

    #[test]
    fn test_parse_time_units() {
        assert_eq!(parse_time("100", "ns"), Some(100.0));
        assert_eq!(parse_time("1.5", "µs"), Some(1500.0));
        assert_eq!(parse_time("2.5", "ms"), Some(2_500_000.0));
        assert_eq!(parse_time("1", "s"), Some(1_000_000_000.0));
    }
}
