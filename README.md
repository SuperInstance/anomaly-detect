# Anomaly Detection

**Anomaly detection** identifies data points, events, or observations that deviate significantly from the dataset's normal patterns.

## Why It Matters

Anomalies signal fraud, system failures, security breaches, or novel phenomena. Detection methods range from statistical (z-score, IQR) to ML-based (isolation forests, autoencoders) to domain-specific rules.

## How It Works

Implements multiple detectors: statistical (z-score, modified z-score, IQR), distance-based (k-NN, LOF), distribution-based (Gaussian fit, GMM), and time-series (STL decomposition, exponentially weighted moving average).

## Usage

```toml
[dependencies]
anomaly-detect = "0.1.0"
```

```rust
use anomaly_detect;

// See examples/ directory for detailed usage
```

## API

- `AnomalyReport` (lib.rs)
- `zscore_detect` (lib.rs)
- `iqr_detect` (lib.rs)
- `modified_zscore_detect` (lib.rs)

## Architecture

This crate is part of the **[SuperInstance](https://github.com/SuperInstance)** ecosystem — a conservation-law-based framework for fleet coordination, ternary computation, and distributed agent systems.

### Related Crates

- [`superinstance-core`](https://github.com/SuperInstance/superinstance-core) — Core conservation law (γ + η = C)
- [`superinstance-harness`](https://github.com/SuperInstance/superinstance-harness) — Build harness and self-improving loop
- [`fleet-coordinator`](https://github.com/SuperInstance/fleet-coordinator) — Fleet-level coordination

## References

- [SuperInstance Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)
- [Conservation Law Paper](https://github.com/SuperInstance/SuperInstance/blob/main/docs/conservation-law.md)

## License

MIT
