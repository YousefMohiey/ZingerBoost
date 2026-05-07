use serde::{Deserialize, Serialize};
use std::sync::mpsc;

/// Progress update during benchmark execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkProgress {
    pub percent: u8,
    pub message: String,
}

/// Result of a benchmark run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub score: f64,
    pub duration_ms: u64,
    pub details: String,
}

/// Trait for pluggable benchmarks
pub trait Benchmark: Send + Sync {
    fn name(&self) -> &str;
    fn run(
        &self,
        tx: mpsc::Sender<BenchmarkProgress>,
    ) -> Result<BenchmarkResult, crate::errors::BenchmarkError>;
}
