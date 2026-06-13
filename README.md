# Anomaly Detection

**Anomaly Detection** is a Rust library implementing three statistical outlier-detection methods — Z-score, Interquartile Range (IQR), and Modified Z-score (MAD-based) — each with different robustness properties for non-normal distributions.

## Why It Matters

Anomaly detection is the computational backbone of fraud detection, system monitoring, sensor validation, and scientific data cleaning. The choice of method matters enormously: Z-score assumes a normal distribution and fails catastrophically on skewed data; IQR is distribution-free but coarse; Modified Z-score (based on Median Absolute Deviation) provides 50% breakdown point robustness — half the data can be corrupted before the estimator fails. Understanding when to use each method is a core data science competency. This library provides clean, tested implementations of all three, enabling comparative analysis and ensemble approaches.

## How It Works

**Z-score Method:**
Computes how many standard deviations each point is from the mean:

```
z_i = |x_i − μ| / σ
```

Anomaly if z_i > threshold (typically 3.0). Assumes normality; μ and σ are both sensitive to outliers (breakdown point 0%). Time: O(n) for mean, O(n) for std, O(n) for scoring = O(n) total.

**IQR Method:**
Distribution-free approach using quartiles:

```
IQR = Q3 − Q1
lower = Q1 − k × IQR
upper = Q3 + k × IQR
```

Anomaly if x_i < lower or x_i > upper (typically k = 1.5). Requires sorting: O(n log n). More robust than Z-score (breakdown point 25%) but loses information about the distribution shape.

**Modified Z-score (MAD-based):**
Replaces mean and std with the more robust median and MAD (Median Absolute Deviation):

```
MAD = median(|x_i − median(x)|)
modified_z_i = |0.6745 × (x_i − median) / MAD|
```

The constant 0.6745 is the 75th percentile of the standard normal, making modified Z-scores comparable to standard Z-scores under normality. Breakdown point: 50% — the maximum possible. Time: O(n log n) for median (via sort) + O(n log n) for MAD = O(n log n).

**Comparison:**

| Method | Breakdown Point | Distribution | Time | Sensitivity |
|--------|----------------|-------------|------|-------------|
| Z-score | 0% | Assumes normal | O(n) | High (mean + std both affected) |
| IQR | 25% | Distribution-free | O(n log n) | Medium |
| Modified Z-score | 50% | Symmetric preferred | O(n log n) | Low (median + MAD both robust) |

## Quick Start

```rust
fn main() {
    let data = vec![1.0, 1.1, 0.9, 1.05, 0.95, 100.0];

    let z_report = zscore_detect(&data, 3.0);
    println!("Z-score anomalies: {:?}", z_report.indices);

    let iqr_report = iqr_detect(&data, 1.5);
    println!("IQR anomalies: {:?}", iqr_report.indices);

    let mad_report = modified_zscore_detect(&data, 3.5);
    println!("Modified Z-score anomalies: {:?}", mad_report.indices);
}
```

## API

| Function | Parameters | Returns |
|----------|-----------|---------|
| `zscore_detect` | data, threshold | AnomalyReport with indices, scores, threshold |
| `iqr_detect` | data, k (multiplier) | AnomalyReport with indices, distances, k |
| `modified_zscore_detect` | data, threshold | AnomalyReport with indices, scores, threshold |

## Architecture Notes

Anomaly detection provides the **statistical monitoring layer** for γ + η = C conservation. When agent behaviors deviate from expected conservation ratios, the anomaly detector flags them for inspection. The MAD-based method is preferred because conservation-law violations are themselves outliers that would corrupt mean-based detection.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

1. Tukey, J.W. (1977). *Exploratory Data Analysis*. Addison-Wesley. (IQR method.)
2. Iglewicz, B. & Hoaglin, D. (1993). *How to Detect and Handle Outliers*. ASQC Basic References in Quality Control.

## License

MIT
