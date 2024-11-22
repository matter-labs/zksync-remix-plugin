use std::{
    fmt,
    time::{Duration, Instant},
};
use vise::*;

#[derive(Debug)]
pub(crate) struct MethodLatency {
    method: Method,
    started_at: Instant,
}

impl MethodLatency {
    pub fn new(method: &'static str) -> Self {
        Self {
            method: Method(method),
            started_at: Instant::now(),
        }
    }
}

impl Drop for MethodLatency {
    fn drop(&mut self) {
        METRICS.latencies[&self.method].observe(self.started_at.elapsed());
    }
}

#[derive(Debug)]
pub(crate) struct CompileLatency {
    started_at: Instant,
}

impl Default for CompileLatency {
    fn default() -> Self {
        Self::new()
    }
}

impl CompileLatency {
    pub fn new() -> Self {
        Self {
            started_at: Instant::now(),
        }
    }
}

impl Drop for CompileLatency {
    fn drop(&mut self) {
        METRICS.compile_latency.observe(self.started_at.elapsed());
    }
}

#[derive(Debug, Clone, Metrics)]
#[metrics(prefix = "remix_plugin")]
pub(crate) struct Metrics {
    /// Latency for API methods.
    #[metrics(buckets = Buckets::LATENCIES)]
    pub latencies: Family<Method, Histogram<Duration>>,
    /// Latency for compilation.
    #[metrics(buckets = Buckets::LATENCIES)]
    pub compile_latency: Histogram<Duration>,
}

/// API method name
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EncodeLabelSet, EncodeLabelValue)]
#[metrics(label = "method")]
pub(crate) struct Method(pub &'static str);

impl fmt::Display for Method {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

#[vise::register]
pub(crate) static METRICS: Global<Metrics> = Global::new();
