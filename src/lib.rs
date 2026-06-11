//! Anomaly detection utilities.
//!
//! Provides Z-score, IQR, and modified Z-score based anomaly detection.

use std::f64;

/// Result of an anomaly detection pass.
#[derive(Debug, Clone)]
pub struct AnomalyReport {
    pub indices: Vec<usize>,
    pub scores: Vec<f64>,
    pub threshold: f64,
}

/// Detect anomalies using Z-score method.
pub fn zscore_detect(data: &[f64], threshold: f64) -> AnomalyReport {
    let n = data.len();
    if n == 0 {
        return AnomalyReport { indices: vec![], scores: vec![], threshold };
    }
    let mean = data.iter().sum::<f64>() / n as f64;
    let std_dev = {
        let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
        variance.sqrt().max(1e-12)
    };
    let scores: Vec<f64> = data.iter().map(|x| ((x - mean) / std_dev).abs()).collect();
    let indices: Vec<usize> = scores.iter().enumerate()
        .filter(|(_, s)| **s > threshold)
        .map(|(i, _)| i)
        .collect();
    AnomalyReport { indices, scores, threshold }
}

/// Detect anomalies using IQR method.
pub fn iqr_detect(data: &[f64], k: f64) -> AnomalyReport {
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = sorted.len();
    if n < 4 {
        return AnomalyReport { indices: vec![], scores: vec![], threshold: 0.0 };
    }
    let q1 = sorted[n / 4];
    let q3 = sorted[3 * n / 4];
    let iqr = q3 - q1;
    let lower = q1 - k * iqr;
    let upper = q3 + k * iqr;
    let mut indices = vec![];
    let mut scores = vec![];
    for (i, &val) in data.iter().enumerate() {
        if val < lower || val > upper {
            let dist = if val < lower { lower - val } else { val - upper };
            indices.push(i);
            scores.push(dist);
        }
    }
    AnomalyReport { indices, scores, threshold: k }
}

/// Detect anomalies using modified Z-score (MAD-based).
pub fn modified_zscore_detect(data: &[f64], threshold: f64) -> AnomalyReport {
    let n = data.len();
    if n == 0 {
        return AnomalyReport { indices: vec![], scores: vec![], threshold };
    }
    let median = {
        let mut s = data.to_vec();
        s.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if n % 2 == 0 { (s[n / 2 - 1] + s[n / 2]) / 2.0 } else { s[n / 2] }
    };
    let mad = {
        let mut deviations: Vec<f64> = data.iter().map(|x| (x - median).abs()).collect();
        deviations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let m = deviations.len();
        if m % 2 == 0 { (deviations[m / 2 - 1] + deviations[m / 2]) / 2.0 } else { deviations[m / 2] }
    };
    let mad_scaled = (mad * 1.4826).max(1e-12);
    let scores: Vec<f64> = data.iter().map(|x| (0.6745 * (x - median) / mad_scaled).abs()).collect();
    let indices: Vec<usize> = scores.iter().enumerate()
        .filter(|(_, s)| **s > threshold)
        .map(|(i, _)| i)
        .collect();
    AnomalyReport { indices, scores, threshold }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zscore_no_anomalies() {
        let data = vec![1.0, 1.1, 0.9, 1.05, 0.95];
        let report = zscore_detect(&data, 3.0);
        assert!(report.indices.is_empty());
    }

    #[test]
    fn test_zscore_with_anomaly() {
        let data = vec![1.0, 1.1, 0.9, 100.0];
        let report = zscore_detect(&data, 2.0);
        assert!(report.indices.contains(&3));
    }

    #[test]
    fn test_iqr_detect() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 100.0];
        let report = iqr_detect(&data, 1.5);
        assert!(report.indices.contains(&5));
    }

    #[test]
    fn test_modified_zscore() {
        let data = vec![10.0, 10.1, 10.2, 9.9, 50.0];
        let report = modified_zscore_detect(&data, 3.5);
        assert!(report.indices.contains(&4));
    }
}
